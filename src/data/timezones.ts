// Compact IANA timezone catalog with searchable RU/UA city/country aliases.
// Used by wizard search-as-you-type and CLI --tz=GMT±N or IANA.

export interface TzEntry {
  iana: string;
  /** GMT offset in winter (informational; DST handled by Intl) */
  gmtWinter: string;  // "GMT+3"
  city: string;       // human display in russian
  country: string;
  aliases: string[];  // for fuzzy search
}

export const TIMEZONES: TzEntry[] = [
  { iana: "Europe/Kaliningrad", gmtWinter: "GMT+2", city: "Калининград", country: "Россия", aliases: ["калининград", "kaliningrad", "rus"] },
  { iana: "Europe/Moscow", gmtWinter: "GMT+3", city: "Москва", country: "Россия", aliases: ["москва", "msk", "moscow", "питер", "санкт-петербург", "spb", "rus"] },
  { iana: "Europe/Samara", gmtWinter: "GMT+4", city: "Самара", country: "Россия", aliases: ["самара", "samara", "ижевск"] },
  { iana: "Asia/Yekaterinburg", gmtWinter: "GMT+5", city: "Екатеринбург", country: "Россия", aliases: ["екб", "yekaterinburg", "пермь", "уфа", "челябинск"] },
  { iana: "Asia/Omsk", gmtWinter: "GMT+6", city: "Омск", country: "Россия", aliases: ["омск", "omsk"] },
  { iana: "Asia/Novosibirsk", gmtWinter: "GMT+7", city: "Новосибирск", country: "Россия", aliases: ["нск", "новосибирск", "novosibirsk", "томск", "красноярск"] },
  { iana: "Asia/Irkutsk", gmtWinter: "GMT+8", city: "Иркутск", country: "Россия", aliases: ["иркутск", "irkutsk", "улан-удэ"] },
  { iana: "Asia/Yakutsk", gmtWinter: "GMT+9", city: "Якутск", country: "Россия", aliases: ["якутск", "yakutsk", "чита"] },
  { iana: "Asia/Vladivostok", gmtWinter: "GMT+10", city: "Владивосток", country: "Россия", aliases: ["владивосток", "vladivostok", "хабаровск"] },
  { iana: "Asia/Magadan", gmtWinter: "GMT+11", city: "Магадан", country: "Россия", aliases: ["магадан", "magadan", "сахалин"] },
  { iana: "Asia/Kamchatka", gmtWinter: "GMT+12", city: "Камчатка", country: "Россия", aliases: ["камчатка", "kamchatka", "петропавловск"] },

  { iana: "Europe/Kyiv", gmtWinter: "GMT+2", city: "Київ", country: "Україна", aliases: ["киев", "київ", "kyiv", "kiev", "ua", "украина", "україна", "львов", "львів", "одесса", "одеса", "харьков", "харків"] },

  { iana: "Europe/Minsk", gmtWinter: "GMT+3", city: "Минск", country: "Беларусь", aliases: ["минск", "minsk", "бел", "беларусь", "by"] },
  { iana: "Asia/Almaty", gmtWinter: "GMT+5", city: "Алматы", country: "Казахстан", aliases: ["алматы", "almaty", "kz", "казахстан", "астана", "нур-султан"] },
  { iana: "Asia/Tashkent", gmtWinter: "GMT+5", city: "Ташкент", country: "Узбекистан", aliases: ["ташкент", "tashkent", "uz", "узбекистан"] },
  { iana: "Asia/Bishkek", gmtWinter: "GMT+6", city: "Бишкек", country: "Кыргызстан", aliases: ["бишкек", "bishkek", "kg"] },
  { iana: "Asia/Tbilisi", gmtWinter: "GMT+4", city: "Тбилиси", country: "Грузия", aliases: ["тбилиси", "tbilisi", "ge", "грузия"] },
  { iana: "Asia/Yerevan", gmtWinter: "GMT+4", city: "Ереван", country: "Армения", aliases: ["ереван", "yerevan", "am", "армения"] },
  { iana: "Asia/Baku", gmtWinter: "GMT+4", city: "Баку", country: "Азербайджан", aliases: ["баку", "baku", "az", "азербайджан"] }
];

export function findTzByQuery(q: string, limit = 8): TzEntry[] {
  const norm = q.trim().toLowerCase();
  if (!norm) return TIMEZONES.slice(0, limit);
  return TIMEZONES.filter(t => {
    if (t.iana.toLowerCase().includes(norm)) return true;
    if (t.city.toLowerCase().includes(norm)) return true;
    if (t.country.toLowerCase().includes(norm)) return true;
    if (t.gmtWinter.toLowerCase().includes(norm)) return true;
    return t.aliases.some(a => a.includes(norm));
  }).slice(0, limit);
}

export function findTzByGmtOffset(offset: number): TzEntry | undefined {
  const target = `GMT+${offset}`;
  const targetNeg = `GMT${offset}`;
  return TIMEZONES.find(t => t.gmtWinter === target || t.gmtWinter === targetNeg);
}

/** Parse "--tz=GMT+3" or "--tz=Europe/Moscow" or "--tz=+3" to IANA. */
export function parseTzFlag(value: string): string | undefined {
  if (!value) return undefined;
  const v = value.trim();
  // direct IANA
  if (v.includes("/")) return TIMEZONES.find(t => t.iana.toLowerCase() === v.toLowerCase())?.iana ?? v;
  // GMT±N
  const m = v.match(/^(?:GMT|UTC|gmt|utc)?\s*([+-]?\d{1,2})$/);
  if (m) {
    const off = parseInt(m[1]!, 10);
    return findTzByGmtOffset(off)?.iana;
  }
  // search by city/country
  return findTzByQuery(v, 1)[0]?.iana;
}

export function defaultTzForNationality(nat: "RU" | "UA"): string {
  return nat === "UA" ? "Europe/Kyiv" : "Europe/Moscow";
}
