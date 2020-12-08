require "option_parser"

module Six
  VERSION = "0.1.0"

  def self.solution1(groups)
    answers = [] of Array(Char)

    groups.each do |group|
      answers << group.gsub("\n", "").chars.uniq
    end

    answers.reduce(0) do |acc, answer|
      acc + answer.size
    end
  end

  def self.solution2(groups)
    answers = [] of Set(Char)

    groups.each do |group|
      answer_sets = group.lines(chomp=true).map(&.chars).map(&.to_set)
      answers << answer_sets.reduce(Set.new('a'..'z')) do |acc, answer|
        acc & answer
      end
    end

    answers.flatten.reduce(0) do |acc, answer|
      acc + answer.size
    end
  end

  def self.parse_args
    filename = ""
    OptionParser.parse do |parser|
      parser.banner = "Day Six of AOC"

      parser.on "-v", "--version", "Show version" do
        puts "version #{VERSION}"
        exit
      end
      parser.on "-h", "--help", "Show help" do
        puts parser
        exit
      end
      parser.on "-f FILE", "--filename=FILE", "Filename" do |name|
        filename = name
      end
    end

    filename
  end

  def self.cli
    input = File.read(parse_args)
    puts solution1 input.chomp.split("\n\n")
    puts solution2 input.chomp.split("\n\n")
  end
end

Six.cli
