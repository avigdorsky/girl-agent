import type { StagePreset } from "../types.js";

export const STAGE_PRESETS: StagePreset[] = [
  {
    id: "met-irl-got-tg",
    label: "Встретились в реале — дала тг",
    description: "Только что обменялись тг. Помнит лицо, голос. Лёгкий интерес.",
    defaults: {
      interest: 38, trust: 14, attraction: 30, annoyance: 0, cringeTolerance: 14,
      ignoreChance: 0.12, replyDelaySec: [15, 600]
    }
  },
  {
    id: "tg-given-cold",
    label: "Дала тг, но не убедил отвечать",
    description: "Сомневается. Часто игнорит, отвечает односложно. Нужно добиваться.",
    defaults: {
      interest: 5, trust: 0, attraction: 5, annoyance: 0, cringeTolerance: -10,
      ignoreChance: 0.65, replyDelaySec: [600, 14400]
    }
  },
  {
    id: "tg-given-warming",
    label: "Дала тг, отвечает осторожно",
    description: "Оттаивает. Отвечает, но коротко. Тестит тебя.",
    defaults: {
      interest: 30, trust: 15, attraction: 25, annoyance: 0, cringeTolerance: 5,
      ignoreChance: 0.18, replyDelaySec: [30, 1200]
    }
  },
  {
    id: "convinced",
    label: "Убедил отвечать стабильно",
    description: "Общаетесь регулярно, флиртует, ещё не виделись после знакомства.",
    defaults: {
      interest: 50, trust: 35, attraction: 45, annoyance: 0, cringeTolerance: 15,
      ignoreChance: 0.07, replyDelaySec: [10, 420]
    }
  },
  {
    id: "first-date-done",
    label: "Сходили один раз",
    description: "Первое свидание было, в подвешенном состоянии — нравится, но не пара.",
    defaults: {
      interest: 60, trust: 45, attraction: 55, annoyance: 0, cringeTolerance: 25,
      ignoreChance: 0.05, replyDelaySec: [8, 300]
    }
  },
  {
    id: "dating-early",
    label: "Только начали встречаться",
    description: "Около месяца вместе. Бабочки, всё внове, но границы ещё хрупкие.",
    defaults: {
      interest: 75, trust: 60, attraction: 70, annoyance: 0, cringeTolerance: 35,
      ignoreChance: 0.02, replyDelaySec: [3, 120]
    }
  },
  {
    id: "dating-stable",
    label: "Пара, общаетесь свободно",
    description: "Стабильные отношения, шутки, бытовуха, доверие.",
    defaults: {
      interest: 80, trust: 80, attraction: 75, annoyance: 0, cringeTolerance: 50,
      ignoreChance: 0.03, replyDelaySec: [3, 240]
    }
  },
  {
    id: "long-term",
    label: "Давно вместе",
    description: "Год+ вместе. Иногда раздражение, рутина, глубокое доверие.",
    defaults: {
      interest: 70, trust: 90, attraction: 65, annoyance: 10, cringeTolerance: 60,
      ignoreChance: 0.05, replyDelaySec: [5, 900]
    }
  },
  {
    id: "dumped",
    label: "Отшила (служебное)",
    description: "Не отвечает. Снимается командой :reset.",
    defaults: {
      interest: -50, trust: -30, attraction: -40, annoyance: 80, cringeTolerance: -50,
      ignoreChance: 1.0, replyDelaySec: [99999, 99999]
    }
  }
];

export function findStage(id: string): StagePreset {
  return STAGE_PRESETS.find(s => s.id === id) ?? STAGE_PRESETS[1]!;
}
