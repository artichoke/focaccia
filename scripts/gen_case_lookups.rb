#!/usr/bin/env ruby
# frozen_string_literal: true

mappings = File.readlines('CaseFolding.txt')

char_mappings = Hash.new { |hash, key| hash[key] = {} }

mappings.each do |line|
  next if line.empty?
  next if line[0] == '#'

  encoded, = line.split('#', 2)
  code, status, map_to = encoded.split(';').map(&:strip)

  next unless %w[C F T].include?(status)

  code = code.to_i(16)
  map_to = map_to.split(' ').map { |char| char.to_i(16) }

  mode = :full
  mode = :turkic if status == 'T'

  char_mappings[code][mode] = map_to
end

range_start = char_mappings.keys.min
last = char_mappings.keys.min

ranges = []

char_mappings.keys.sort.drop(1).each do |key|
  if key == last + 1 \
    && char_mappings[key][:full].length == char_mappings[last][:full].length \
    && char_mappings[key][:full].first == char_mappings[last][:full].first + 1
    last = key
    next
  end
  ranges << { start: range_start, end: last, span: last - range_start + 1 }
  range_start = key
  last = key
end
ranges << { start: range_start, end: last, span: last - range_start + 1 }

ranges.each do |range|
  start = range[:start]
  last = range[:end]

  start_offset = char_mappings[start][:full][0] - start
  (start..last).each do |char|
    char_offset = char_mappings[char][:full][0] - char
    raise "unequal offset in char range: #{range.inspect}" unless char_offset == start_offset
  end
  range[:offset] = start_offset
end

ranges.each do |range|
  puts range.inspect
end

rs = File.open('src/folding/mapping/lookup.rs', 'w')

rs.puts(<<~AUTOGEN)
  use super::{Mapping, Mode};

  #[must_use]
  #[allow(clippy::match_same_arms)]
  #[allow(clippy::too_many_lines)]
  pub fn lookup(c: char, mode: Mode) -> Mapping {
      match c {
          // Turkic mapping in ASCII range
          // 0049; T; 0131; # LATIN CAPITAL LETTER I
          '\\u{0049}' if mode == Mode::Turkic => Mapping::Single(0x0131_u32),
          c if c.is_ascii() => Mapping::Single(c.to_ascii_lowercase() as u32),
AUTOGEN

ranges.each do |range|
  next if range[:end] < 128

  start = range[:start]
  last = range[:end]
  offset = range[:offset]
  mapping = char_mappings[start]
  if mapping.key?(:turkic) && mapping.key?(:full)
    raise unless (last - start).zero?

    char = start.to_s(16).upcase.rjust(4, '0')
    full = mapping[:full].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }
    case full.length
    when 1
      rs.puts "        '\\u{#{char}}' if mode == Mode::Full => Mapping::Single(0x#{full[0]}),"
    when 2
      rs.puts "        '\\u{#{char}}' if mode == Mode::Full => Mapping::Double(0x#{full[0]}, 0x#{full[1]}),"
    when 3
      rs.puts "        '\\u{#{char}}' if mode == Mode::Full => Mapping::Triple(0x#{full[0]}, 0x#{full[1]}}, 0x#{full[2]}),"
    else
      raise "Unsupported mapping length: #{map.inspect} for code #{code}"
    end
    turkic = mapping[:turkic].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }
    case turkic.length
    when 1
      rs.puts "        '\\u{#{char}}' if mode == Mode::Turkic => Mapping::Single(0x#{turkic[0]}),"
    when 2
      rs.puts "        '\\u{#{char}}' if mode == Mode::Turkic => Mapping::Double(0x#{turkic[0]}, 0x#{turkic[1]}),"
    when 3
      rs.puts "        '\\u{#{char}}' if mode == Mode::Turkic => Mapping::Triple(0x#{turkic[0]}, 0x#{turkic[1]}, 0x#{turkic[2]}),"
    else
      raise "Unsupported mapping length: #{map.inspect} for code #{code}"
    end
  elsif mapping.key?(:full) && !offset.nil?
    full = mapping[:full].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }

    base = start.to_s(16).upcase.rjust(4, '0')
    op = 'add'
    op_offset = offset
    if offset.negative?
      op = 'sub'
      op_offset = -offset
    end
    op_offset = op_offset.to_s(16).rjust(4, '0')
    if (last - start).zero? && full.length == 1
      rs.puts "        '\\u{#{base}}' => Mapping::Single(0x#{full[0]}),"
    elsif full.length == 1
      finish = last.to_s(16).upcase.rjust(4, '0')
      rs.puts "        '\\u{#{base}}'..='\\u{#{finish}}' => Mapping::Single((c as u32).wrapping_#{op}(0x#{op_offset})),"
    elsif (last - start).zero? && full.length == 2
      rs.puts "        '\\u{#{base}}' => Mapping::Double(0x#{full[0]}, 0x#{full[1]}),"
    elsif full.length == 2
      finish = last.to_s(16).upcase.rjust(4, '0')
      rs.puts "        '\\u{#{base}}'..='\\u{#{finish}}' => Mapping::Double((c as u32).wrapping_#{op}(0x#{op_offset}), 0x#{full[1]}),"
    elsif (last - start).zero? && full.length == 3
      rs.puts "        '\\u{#{base}}' => Mapping::Triple(0x#{full[0]}, 0x#{full[1]}, 0x#{full[2]}),"
    elsif full.length == 3
      finish = last.to_s(16).upcase.rjust(4, '0')
      rs.puts "        '\\u{#{base}}'..='\\u{#{finish}}' => Mapping::Triple((c as u32).wrapping_#{op}(0x#{op_offset}), 0x#{full[1]}, 0x#{full[2]}),"
    end
  elsif mapping.key?(:full)
    char = start.to_s(16).upcase.rjust(4, '0')
    map = mapping[:full].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }
    case map.length
    when 1
      rs.puts "        '\\u{#{char}}' => Mapping::Single(0x#{map[0]}),"
    when 2
      rs.puts "        '\\u{#{char}}' => Mapping::Double(0x#{map[0]}, 0x#{map[1]}),"
    when 3
      rs.puts "        '\\u{#{char}}' => Mapping::Triple(0x#{map[0]}, 0x#{map[1]}, 0x#{map[2]}),"
    else
      raise "Unsupported mapping length: #{map.inspect} for code #{code}"
    end
  else
    raise 'unsupported full/turkic mapping combination'
  end
end

rs.puts '        _ => Mapping::Single(c as u32),'

rs.puts '    }'
rs.puts '}'
rs.close

rs = File.open('tests/full_fold_exhaustive.rs', 'w')

rs.puts(<<~AUTOGEN)
  use core::char;
  use core::cmp::Ordering;
  use focaccia::{unicode_full_case_eq, unicode_full_casecmp};

  #[must_use]
  #[allow(clippy::too_many_lines)]
  fn lookup_naive(c: char, buf: &mut [u8; 4]) -> &str {
      match c {
AUTOGEN

char_mappings.keys.sort.each do |from|
  mapping = char_mappings[from]

  char = from.to_s(16).upcase.rjust(4, '0')
  full = mapping[:full].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }

  case full.length
  when 1
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\","
  when 2
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\\u{#{full[1]}}\","
  when 3
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\\u{#{full[1]}}\\u{#{full[2]}}\","
  else
    raise "Unsupported mapping length: #{map.inspect} for code #{code}"
  end
end

rs.puts '        _ => c.encode_utf8(buf),'
rs.puts '    }'
rs.puts '}'
rs.puts

rs.puts(<<~TEST)
  #[test]
  fn full_fold_exhaustive() {
      let mut enc = [0; 4];
      let mut buf = [0; 4];
      for codepoint in 0..=0x10FFFF {
          if let Some(ch) = char::from_u32(codepoint) {
              let left = ch.encode_utf8(&mut enc);
              let right = lookup_naive(ch, &mut buf);
              assert!(
                  unicode_full_case_eq(left, right),
                  "Correctness check failed for: {}. Expected: {}. Got: {}.",
                  ch,
                  left,
                  right
              );
              assert!(
                  matches!(unicode_full_casecmp(left, right), Ordering::Equal),
                  "Correctness check failed for: {}. Expected: {}. Got: {}.",
                  ch,
                  left,
                  right
              );
          }
      }
  }
TEST
rs.close

rs = File.open('tests/full_turkic_fold_exhaustive.rs', 'w')

rs.puts(<<~AUTOGEN)
  use core::char;
  use core::cmp::Ordering;
  use focaccia::{unicode_full_turkic_case_eq, unicode_full_turkic_casecmp};

  #[must_use]
  #[allow(clippy::too_many_lines)]
  fn lookup_naive(c: char, buf: &mut [u8; 4]) -> &str {
      match c {
AUTOGEN

char_mappings.keys.sort.each do |from|
  mapping = char_mappings[from]

  char = from.to_s(16).upcase.rjust(4, '0')
  full =
    if mapping[:turkic]
      mapping[:turkic].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }
    else
      mapping[:full].map { |ch| ch.to_s(16).upcase.rjust(4, '0') }
    end

  case full.length
  when 1
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\","
  when 2
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\\u{#{full[1]}}\","
  when 3
    rs.puts "        '\\u{#{char}}' => \"\\u{#{full[0]}}\\u{#{full[1]}}\\u{#{full[2]}}\","
  else
    raise "Unsupported mapping length: #{map.inspect} for code #{code}"
  end
end

rs.puts '        _ => c.encode_utf8(buf),'
rs.puts '    }'
rs.puts '}'
rs.puts

rs.puts(<<~TEST)
  #[test]
  fn full_turkic_fold_exhaustive() {
      let mut enc = [0; 4];
      let mut buf = [0; 4];
      for codepoint in 0..=0x10FFFF {
          if let Some(ch) = char::from_u32(codepoint) {
              let left = ch.encode_utf8(&mut enc);
              let right = lookup_naive(ch, &mut buf);
              assert!(
                  unicode_full_turkic_case_eq(left, right),
                  "Correctness check failed for: {}. Expected: {}. Got: {}.",
                  ch,
                  left,
                  right
              );
              assert!(
                  matches!(unicode_full_turkic_casecmp(left, right), Ordering::Equal),
                  "Correctness check failed for: {}. Expected: {}. Got: {}.",
                  ch,
                  left,
                  right
              );
          }
      }
  }
TEST
rs.close
