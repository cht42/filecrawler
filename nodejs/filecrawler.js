const fs = require("fs");
const path = require("path");

const { Worker } = require("worker_threads");

async function* walk(dir) {
  for await (const d of await fs.promises.opendir(dir)) {
    const entry = path.join(dir, d.name);
    if (d.isDirectory()) yield* walk(entry);
    else if (d.isFile()) yield entry;
  }
}

async function process_file(path) {
  return new Promise((resolve, reject) => {
    const worker = new Worker("./worker.js", { workerData: path });
    worker.on("message", (sha) => resolve([path, sha]));
    worker.on("error", reject);
    worker.on("exit", (code) => {
      if (code !== 0) reject(new Error(`Worker stopped with exit code ${code}`));
    });
  });
}

async function main() {
  const rootDir = process.argv[2];

  const start = Date.now();

  const promises = [];
  for await (const path of walk(rootDir)) {
    promises.push(process_file(path));
  }
  const results = await Promise.all(promises);
  for (const result of results) {
    const [path, sha] = result;
    console.log(`${path}: ${sha}`);
  }

  const end = Date.now();
  console.log(`Took ${(end - start) / 1000} seconds to crawl ${rootDir}`);
}

main();
