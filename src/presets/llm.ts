import type { LLMPreset } from "../types.js";

export const LLM_PRESETS: LLMPreset[] = [
  {
    id: "openai",
    name: "OpenAI",
    proto: "openai",
    defaultModel: "gpt-4o-mini",
    models: ["gpt-4o", "gpt-4o-mini", "gpt-4.1", "gpt-4.1-mini", "o4-mini"]
  },
  {
    id: "anthropic",
    name: "Anthropic",
    proto: "anthropic",
    defaultModel: "claude-sonnet-4-5",
    models: ["claude-sonnet-4-5", "claude-opus-4-1", "claude-haiku-4-5"]
  },
  {
    id: "openrouter",
    name: "OpenRouter",
    proto: "openai",
    baseURL: "https://openrouter.ai/api/v1",
    defaultModel: "anthropic/claude-sonnet-4.5",
    models: [
      "anthropic/claude-sonnet-4.5",
      "openai/gpt-4o",
      "google/gemini-2.5-flash",
      "deepseek/deepseek-chat",
      "x-ai/grok-4"
    ]
  },
  {
    id: "groq",
    name: "Groq",
    proto: "openai",
    baseURL: "https://api.groq.com/openai/v1",
    defaultModel: "llama-3.3-70b-versatile",
    models: ["llama-3.3-70b-versatile", "llama-3.1-8b-instant", "mixtral-8x7b-32768"]
  },
  {
    id: "deepseek",
    name: "DeepSeek",
    proto: "openai",
    baseURL: "https://api.deepseek.com",
    defaultModel: "deepseek-chat",
    models: ["deepseek-chat", "deepseek-reasoner"]
  },
  {
    id: "mistral",
    name: "Mistral",
    proto: "openai",
    baseURL: "https://api.mistral.ai/v1",
    defaultModel: "mistral-large-latest",
    models: ["mistral-large-latest", "mistral-small-latest", "ministral-8b-latest"]
  },
  {
    id: "google",
    name: "Google Gemini",
    proto: "openai",
    baseURL: "https://generativelanguage.googleapis.com/v1beta/openai",
    defaultModel: "gemini-2.5-flash",
    models: ["gemini-2.5-pro", "gemini-2.5-flash", "gemini-2.5-flash-lite"],
    hint: "Gemini via OpenAI-compatible endpoint"
  },
  {
    id: "xai",
    name: "xAI Grok",
    proto: "openai",
    baseURL: "https://api.x.ai/v1",
    defaultModel: "grok-4",
    models: ["grok-4", "grok-3", "grok-3-mini"]
  },
  {
    id: "together",
    name: "Together AI",
    proto: "openai",
    baseURL: "https://api.together.xyz/v1",
    defaultModel: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
    models: [
      "meta-llama/Llama-3.3-70B-Instruct-Turbo",
      "Qwen/Qwen2.5-72B-Instruct-Turbo",
      "deepseek-ai/DeepSeek-V3"
    ]
  },
  {
    id: "fireworks",
    name: "Fireworks",
    proto: "openai",
    baseURL: "https://api.fireworks.ai/inference/v1",
    defaultModel: "accounts/fireworks/models/llama-v3p3-70b-instruct",
    models: [
      "accounts/fireworks/models/llama-v3p3-70b-instruct",
      "accounts/fireworks/models/qwen2p5-72b-instruct",
      "accounts/fireworks/models/deepseek-v3"
    ]
  },
  {
    id: "perplexity",
    name: "Perplexity",
    proto: "openai",
    baseURL: "https://api.perplexity.ai",
    defaultModel: "sonar-pro",
    models: ["sonar-pro", "sonar", "sonar-reasoning"]
  },
  {
    id: "cerebras",
    name: "Cerebras",
    proto: "openai",
    baseURL: "https://api.cerebras.ai/v1",
    defaultModel: "llama-3.3-70b",
    models: ["llama-3.3-70b", "llama-4-scout-17b-16e-instruct", "qwen-3-32b"]
  },
  {
    id: "claudehub",
    name: "ClaudeHub",
    proto: "anthropic",
    baseURL: "https://api.claudehub.fun",
    defaultModel: "claude-sonnet-4.6",
    models: ["claude-opus-4.7", "claude-opus-4.6", "claude-opus-4.5", "claude-sonnet-4.6", "claude-sonnet-4.5", "claude-haiku-4.5", "gpt-5.4", "gpt-5.5"],
    hint: "ClaudeHub proxy for Anthropic & OpenAI (РФ, СБП, крипта)"
  },
  {
    id: "custom-openai",
    name: "Custom (OpenAI-compatible)",
    proto: "openai",
    defaultModel: "",
    custom: true,
    hint: "Provide base URL + model name"
  },
  {
    id: "custom-anthropic",
    name: "Custom (Anthropic-compatible)",
    proto: "anthropic",
    defaultModel: "",
    custom: true,
    hint: "Provide base URL + model name"
  }
];

export function findPreset(id: string): LLMPreset | undefined {
  return LLM_PRESETS.find(p => p.id === id);
}
