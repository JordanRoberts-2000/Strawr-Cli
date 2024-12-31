import { pino } from "pino";

const logger = pino({
  serializers: {
    err: (err) => err.message,
  },
  transport: {
    target: "pino-pretty",
  },
  base: null,
  timestamp: false,
});

export default logger;
