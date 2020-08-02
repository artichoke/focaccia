#!/usr/bin/env ruby
# frozen_string_literal: true

mappings = File.readlines('CaseFolding.txt')

rs = File.open('src/folding/mapping/turkic.rs', 'w')

rs.puts(<<~AUTOGEN)
  use super::Mapping;

  #[allow(clippy::match_same_arms)]
  #[must_use]
  pub fn lookup(c: char) -> Mapping {
      match c {
AUTOGEN

found_turkic_mapping = false
mappings = mappings.reverse_each.reject do |line|
  next true if line.empty?
  next true if line[0] == '#'

  if found_turkic_mapping
    found_turkic_mapping = false
    next true
  end

  encoded, = line.split('#', 2)
  _, status, = encoded.split(';').map(&:strip)
  found_turkic_mapping = status == 'T'
  false
end
mappings = mappings.reverse

last = 0

mappings.each do |line|
  next if line.empty?
  next if line[0] == '#'

  encoded, = line.split('#', 2)
  code, status, map_to = encoded.split(';').map(&:strip)

  next unless %w[C F T].include?(status)

  unless last == code.to_i(16)
    rs.print "        '\\u{#{last.to_s(16).rjust(4, '0').upcase}}'..='\\u{#{code}}'"
    rs.puts " if c < '\\u{#{code}}' => Mapping::Single(c),"
  end
  last = code.to_i(16) + 1

  map = map_to.split(' ')
  case map.length
  when 1
    rs.puts "        '\\u{#{code}}' => Mapping::Single('\\u{#{map[0]}}'),"
  when 2
    rs.puts "        '\\u{#{code}}' => Mapping::Double('\\u{#{map[0]}}', '\\u{#{map[1]}}'),"
  when 3
    rs.puts "        '\\u{#{code}}' => Mapping::Triple('\\u{#{map[0]}}', '\\u{#{map[1]}}', '\\u{#{map[2]}}'),"
  else
    raise "Unsupported mapping length: #{map.inspect} for code #{code}"
  end
end

rs.puts '        _ => Mapping::Single(c),'

rs.puts '   }'
rs.puts '}'
