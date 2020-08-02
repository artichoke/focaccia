# frozen_string_literal: true

require 'fileutils'
require 'open-uri'

task default: :lint

desc 'Lint and format'
task lint: %i[lint:format lint:clippy lint:rubocop]

namespace :lint do
  desc 'Run Clippy'
  task :clippy do
    roots = Dir.glob('**/{build,lib,main}.rs')
    roots.each do |root|
      FileUtils.touch(root)
    end
    sh 'cargo clippy --workspace --all-targets --all-features'
  end

  desc 'Run RuboCop'
  task :rubocop do
    sh 'rubocop -a'
  end

  desc 'Format sources'
  task :format do
    sh 'cargo fmt -- --color=auto'
    sh "npx prettier --write '**/*'"
  end
end

desc 'Generate Rust API documentation'
task :doc do
  ENV['RUSTDOCFLAGS'] = '-D warnings'
  sh 'rustup run --install nightly cargo doc --workspace'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  ENV['RUSTDOCFLAGS'] = '-D warnings'
  sh 'rustup run --install nightly cargo doc --workspace --open'
end

desc 'Run Artichoke unit tests'
task :test do
  sh 'cargo test --workspace'
end

namespace :unicode do
  desc 'Rebuild Rust generated Rust sources from Unicode data'
  task :build do
    sh 'ruby scripts/gen_case_lookups.rb'
  end

  desc 'Update Unicode data'
  task :update do
    open('https://www.unicode.org/Public/UCD/latest/ucd/CaseFolding.txt') do |data|
      File.open('CaseFolding.txt', 'w') do |file|
        data.each_line do |line|
          file.write(line)
        end
      end
    end
  end
end
