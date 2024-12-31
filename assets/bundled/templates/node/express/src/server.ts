import express, { Request, Response } from "express";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));

app.get("/", (req: Request, res: Response) => {
  res.status(200).send("Hello world");
});

const port = 3000;
const server = app.listen(port, () => {
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
