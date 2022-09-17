
#include <chrono>
#include <filesystem>
#include <iostream>
#include <sstream>
#include <string>
#include <tuple>

#include "openssl/sha.h"

#include "ctpl.h"

namespace fs = std::filesystem;

std::string to_hex(unsigned char s) {
  std::stringstream ss;
  ss << std::hex << (int)s;
  return ss.str();
}

std::string sha256(int id, fs::directory_entry entry) {
  (void)id;
  FILE *file = fopen(entry.path().c_str(), "rb");
  if (!file)
    return "";

  const int buff_size = 30 * 1024;
  unsigned char *buffer = (unsigned char *)std::malloc(buff_size);
  if (!buffer)
    return "";

  SHA256_CTX sha256;
  unsigned char hash[SHA256_DIGEST_LENGTH];
  int bytes_read = 0;

  SHA256_Init(&sha256);

  while ((bytes_read = fread(buffer, 1, buff_size, file)))
    SHA256_Update(&sha256, buffer, bytes_read);

  SHA256_Final(hash, &sha256);

  fclose(file);
  free(buffer);

  std::string output = "";
  for (int i = 0; i < SHA256_DIGEST_LENGTH; i++)
    output += to_hex(hash[i]);
  return output;
}

int main(int ac, char **av) {
  std::string root_dir;

  if (ac != 2) {
    std::cerr << "Error: Wrong number of arguments\n";
    exit(1);
  }

  std::chrono::steady_clock::time_point begin =
      std::chrono::steady_clock::now();

  root_dir = std::string(av[1]);

  ctpl::thread_pool pool(8);
  std::vector<std::tuple<fs::path, std::future<std::string>>> results;

  for (const auto &entry : fs::recursive_directory_iterator(root_dir))
    if (entry.is_regular_file()) {
      results.push_back(
          std::make_tuple(entry.path(), pool.push(sha256, entry)));
    }

  for (auto &res : results) {
    std::cout << std::get<0>(res) << ": " << std::get<1>(res).get()
              << std::endl;
  }

  std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
  std::cout
      << "Took "
      << std::chrono::duration_cast<std::chrono::seconds>(end - begin).count()
      << " seconds to crawl " << root_dir << std::endl;

  return 0;
}