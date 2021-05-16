import { contextBridge } from "electron";
import { sendDynamoDBRequest } from "./rendererAPI/dynamodb/sendDynamoDBRequest";
import { RendererAPI } from "./rendererAPI/types";

const api: RendererAPI = {
  sendDynamoDBRequest,
};

contextBridge.exposeInMainWorld("api", api);
