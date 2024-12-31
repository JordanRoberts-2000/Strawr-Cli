import http from "http";

const server = http.createServer((req, res) => {
  if (req.url === "/") {
    if (req.method === "GET") {
      res.writeHead(200, { "Content-Type": "text/plain" });
      res.end("Hello World");
    }
  }
});

const port = 3000;
server.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});

server.on("error", (error: NodeJS.ErrnoException) => {
  if (error.code === "EADDRINUSE") {
    console.error(`Port ${port} is in use`);
  } else {
    console.error(error, `Failed to start server`);
  }
  process.exit(1);
});
