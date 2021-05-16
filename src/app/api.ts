import { RendererAPI } from "../rendererAPI/types";
export * from "../rendererAPI/types";

export const api = (globalThis as any).api as RendererAPI;
