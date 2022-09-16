import * as fs from "fs";
import * as path from "path";
import * as crypto from "crypto";

const sha256 = (path: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const hash = crypto.createHash("sha256").setEncoding("hex");
    fs.createReadStream(path)
      .once("error", reject)
      .pipe(hash)
      .once("finish", () => resolve(hash.read()));
  });
};

async function* walk(dir: string): AsyncGenerator<string> {
  for await (const d of await fs.promises.opendir(dir)) {
    const entry = path.join(dir, d.name);
    if (d.isDirectory())
      try {
        yield* walk(entry);
      } catch (error) {
        console.error(`Error on directory ${entry}: ${error}}`);
      }
    else if (d.isFile()) yield entry;
  }
}

async function main() {
  const rootDir = process.argv[2];

  const start = Date.now();

  const promises = [];
  for await (const path of walk(rootDir))
    promises.push(sha256(path).then((sha) => console.log(`${path}: ${sha}`)));
  await Promise.all(promises);

  const end = Date.now();
  console.log(`Took ${(end - start) / 1000} seconds to crawl ${rootDir}`);
}

main();
