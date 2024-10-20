import dotenv from "dotenv";

const res = dotenv.config();
if (res.error) {
  console.log("Error loading .env file: {}", res.error);
  process.exit(1);
}
