const fs = require("fs");
const crypto = require("crypto");
const { parentPort, workerData } = require("worker_threads");

const sha256 = (path) => {
  return new Promise((resolve, reject) => {
    const hash = crypto.createHash("sha256").setEncoding("hex");
    fs.createReadStream(path)
      .once("error", reject)
      .pipe(hash)
      .once("finish", () => resolve(hash.read()));
  });
};

const path = workerData;
if (!path) return;
sha256(path).then((sha) => parentPort.postMessage(sha));
