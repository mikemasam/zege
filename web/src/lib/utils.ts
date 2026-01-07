import { clsx, type ClassValue } from "clsx";
import { DateTime } from "luxon";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export default class Utils {
  static parseRelativeTime(str: string): DateTime {
    const value = parseInt(str, 10);
    const unit = str.replace(/\d+/g, "");
    let dt = DateTime.now();
    switch (unit) {
      case "m": // minutes
        dt = dt.minus({ minutes: value });
        break;
      case "h": // hours
        dt = dt.minus({ hours: value });
        break;
      case "d": // days
        dt = dt.minus({ days: value });
        break;
      case "w": // weeks
        dt = dt.minus({ weeks: value });
        break;
      case "M": // months
        dt = dt.minus({ months: value });
        break;
      case "y": // years
        dt = dt.minus({ years: value });
        break;
      default:
        throw new Error("Unknown unit");
    }

    return dt;
  }
}
