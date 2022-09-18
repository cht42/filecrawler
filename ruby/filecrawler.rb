require "digest"

STDOUT.sync = true

if ARGV.length != 1
  puts "Wrong number of arguments"
  exit
end

root_dir = ARGV[0]

def get_sha(path)
  hash = Digest::SHA256.new
  File.open(path) do |file|
    while chunk = file.read(30 * 1024)
      hash << chunk
    end
  end
  return hash.hexdigest
end

def walk(dir)
  Dir.foreach(dir) do |entry|
    path = File.join(dir, entry)
    if entry == "." or entry == ".."
      next
    elsif File.directory?(path)
      walk(path)
    else
      puts "#{path}: #{get_sha(path)}"
    end
  end
end

walk(root_dir)