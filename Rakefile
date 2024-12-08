# frozen_string_literal: true

require 'open-uri'
require 'shellwords'
require 'bundler/audit/task'
require 'rubocop/rake_task'

task default: %i[format lint]

desc 'Lint sources'
task lint: %i[lint:clippy lint:rubocop:autocorrect]

namespace :lint do
  RuboCop::RakeTask.new(:rubocop)

  desc 'Lint Rust sources with Clippy'
  task :clippy do
    sh 'cargo clippy --workspace --all-features --all-targets'
  end

  desc 'Lint Rust sources with Clippy restriction pass (unenforced lints)'
  task :'clippy:restriction' do
    lints = [
      'clippy::dbg_macro',
      'clippy::get_unwrap',
      'clippy::indexing_slicing',
      'clippy::panic',
      'clippy::print_stdout',
      'clippy::expect_used',
      'clippy::unwrap_used',
      'clippy::todo',
      'clippy::unimplemented',
      'clippy::unreachable'
    ]
    command = ['cargo', 'clippy', '--'] + lints.flat_map { |lint| ['-W', lint] }
    sh command.shelljoin
  end
end

desc 'Format sources'
task format: %i[format:rust format:text]

namespace :format do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npm run fmt'
  end
end

desc 'Format sources'
task fmt: %i[fmt:rust fmt:text]

namespace :fmt do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npm run fmt'
  end
end

desc 'Build Rust workspace'
task :build do
  sh 'cargo build --workspace'
end

desc 'Generate Rust API documentation'
task :doc do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace --open'
end

desc 'Run Focaccia unit tests'
task :test do
  sh 'cargo test --workspace'
end

namespace :unicode do
  desc 'Rebuild Rust generated Rust sources from Unicode data'
  task :build do
    ruby 'scripts/gen_case_lookups.rb'
  end

  # Per the Unicode Terms of Use -- https://www.unicode.org/copyright.html --
  # data found under `https://www.unicode.org/Public/` are considered Unicode
  # Data Files and are subject to these constaints:
  #
  # - Except where otherwise more broadly permitted or licensed:
  #   - you may not make copies of or modifications to Unicode Products for
  #     public distribution, or incorporate Unicode Products in whole or in part
  #     into any product or publication, or otherwise publicly distribute them,
  #     without the express written permission of Unicode, and
  #   - you may not copy or extract fonts or font data from any Unicode Products,
  #     including but not limited to Unicode Code Charts.
  # - All Unicode Data Files and Unicode Software are subject to the terms and
  #   conditions of the free and open-source Unicode License v3, unless otherwise
  #   indicated by specific restriction, permission, or license identified at the
  #   point of release or in such software, data file, or other documentation.
  #
  # The Unicode License v3, which can be found at <https://www.unicode.org/license.txt>
  # is included in this repository. The license requires one of:
  #
  # (a) this copyright and permission notice appear with all copies of the Data
  #     Files or Software
  # (b) this copyright and permission notice appear in associated Documentation
  #
  # `focaccia` distributes this license as `LICENSE-UNICODE` in crate bundles
  # and includes `AND Unicode-3.0` in the `Cargo.toml` SPDX license expression.
  # See: https://spdx.org/licenses/Unicode-3.0.html.
  #
  # Updates to Unicode Data Files performed by this `rake` task also update the
  # embedded license.
  desc 'Update Unicode data'
  task :update do
    URI.parse('https://www.unicode.org/license.txt').open do |data|
      IO.copy_stream(data, 'LICENSE-UNICODE')
    end
    URI.parse('https://www.unicode.org/Public/UCD/latest/ucd/CaseFolding.txt').open do |data|
      IO.copy_stream(data, 'CaseFolding.txt')
    end
  end
end

Bundler::Audit::Task.new

namespace :release do
  link_check_files = FileList.new('**/*.md') do |f|
    f.exclude('node_modules/**/*')
    f.exclude('**/target/**/*')
    f.exclude('**/vendor/**/*')
    f.include('*.md')
    f.include('**/vendor/*.md')
  end

  link_check_files.sort.uniq.each do |markdown|
    desc 'Check for broken links in markdown files'
    task markdown_link_check: markdown do
      command = ['npx', 'markdown-link-check', '--config', '.github/markdown-link-check.json', markdown]
      sh command.shelljoin
      sleep(rand(1..5))
    end
  end
end
