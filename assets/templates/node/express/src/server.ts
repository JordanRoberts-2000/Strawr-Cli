import express, { Request, Response } from "express";

const app = express();

app.use(express.json());
app.use(express.urlencoded({ extended: false }));

app.get("/", (req: Request, res: Response) => {
  res.status(200).send("Hello world");
});

const port = 3000;
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
