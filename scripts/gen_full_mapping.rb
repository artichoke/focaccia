#!/usr/bin/env ruby
# frozen_string_literal: true

mappings = File.readlines('CaseFolding.txt')

rs = File.open('src/folding/full.rs', 'w')

rs.puts '// This file is generated. See `gen_full_mapping.rb`.'
rs.puts

rs.puts '/// Compare two strings with Full Unicode case folding.'
rs.puts '///'
rs.puts '/// This function is implemented with a lookup table generated from Unicode case'
rs.puts '/// folding tables.'
rs.puts '#[must_use]'
rs.puts '#[allow(clippy::match_same_arms)]'
rs.puts 'pub fn casecmp(left: &str, right: &str) -> bool {'
rs.puts 'let mut left = left.chars();'
rs.puts 'let mut right = right.chars();'
rs.puts 'loop {'
rs.puts 'match (left.next(), right.next()) {'
rs.puts '(None, None) => return true,'
rs.puts '(Some(_), None) | (None, Some(_)) => return false,'
rs.puts '(Some(left), Some(right)) if left == right => continue,'

mappings.each do |line|
  next if line.empty?
  next if line[0] == '#'

  encoded, = line.split('#', 2)
  code, status, map_to = encoded.split(';').map(&:strip)

  next unless %w[C F].include?(status)

  map_to_left = map_to.split(' ')
  map_to_right = map_to.split(' ')

  rs.puts "(Some('\\u{#{code}}'), Some('\\u{#{map_to_left.shift}}')) "
  unless map_to_left.empty?
    rs.puts 'if '
    rs.puts "matches!(right.next(), Some('\\u{#{map_to_left.shift}}'))"
    rs.puts " && matches!(right.next(), Some('\\u{#{map_to_left.shift}}'))" until map_to_left.empty?
  end
  rs.puts '=> continue,'

  rs.puts "(Some('\\u{#{map_to_right.shift}}'), Some('\\u{#{code}}')) "
  unless map_to_right.empty?
    rs.puts 'if '
    rs.puts "matches!(left.next(), Some('\\u{#{map_to_right.shift}}'))"
    rs.puts " && matches!(left.next(), Some('\\u{#{map_to_right.shift}}'))" until map_to_right.empty?
  end
  rs.puts '=> continue,'
end

rs.puts '_ => return false,'

rs.puts '}'
rs.puts '}'
rs.puts '}'

rs.puts

rs.puts(<<~TESTS)
  #[cfg(test)]
  mod tests {
      use super::casecmp;

      #[test]
      fn compares_symbols_without_regard_to_case() {
          assert!(!casecmp("abcdef", "abcde"));
          assert!(casecmp("aBcDeF", "abcdef"));
          assert!(!casecmp("abcdef", "abcdefg"));
          assert!(casecmp("abcdef", "ABCDEF"));
      }

      #[test]
      fn doesent_consider_non_ascii_chars_equal_that_arent() {
          // -- UTF-8 --
          let upper_a_tilde = "Ã";
          let lower_a_tilde = "ã";
          let upper_a_umlaut = "Ä";
          let lower_a_umlaut = "ä";

          // From `spec/core/symbol/casecmp_spec.rb`:
          //
          // ```ruby
          // lower_a_tilde.casecmp?(lower_a_umlaut).should_not == true
          // lower_a_umlaut.casecmp?(lower_a_tilde).should_not == true
          // upper_a_tilde.casecmp?(upper_a_umlaut).should_not == true
          // upper_a_umlaut.casecmp?(upper_a_tilde).should_not == true
          // ```
          assert!(!casecmp(lower_a_tilde, lower_a_umlaut));
          assert!(!casecmp(lower_a_umlaut, lower_a_tilde));
          assert!(!casecmp(upper_a_tilde, upper_a_umlaut));
          assert!(!casecmp(upper_a_umlaut, upper_a_tilde));
      }

      #[test]
      fn does_case_mapping_for_unicode_chars() {
          // -- UTF-8 --
          let upper_a_tilde = "Ã";
          let lower_a_tilde = "ã";
          let upper_a_umlaut = "Ä";
          let lower_a_umlaut = "ä";

          // From `spec/core/symbol/casecmp_spec.rb`:
          //
          // ```ruby
          // upper_a_tilde.casecmp?(lower_a_tilde).should == true
          // upper_a_umlaut.casecmp?(lower_a_umlaut).should == true
          // lower_a_tilde.casecmp?(upper_a_tilde).should == true
          // lower_a_umlaut.casecmp?(upper_a_umlaut).should == true
          // ```
          assert!(casecmp(upper_a_tilde, lower_a_tilde));
          assert!(casecmp(upper_a_umlaut, lower_a_umlaut));
          assert!(casecmp(lower_a_tilde, upper_a_tilde));
          assert!(casecmp(lower_a_umlaut, upper_a_umlaut));
      }
  }
TESTS
