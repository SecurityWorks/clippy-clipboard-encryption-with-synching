import { invoke } from "@tauri-apps/api";
import {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { exit } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
import { Hotkey } from "../@types";
import AppStore from "../store/AppStore";
import ClipboardStore from "../store/ClipboardStore";
import HotkeyStore from "../store/HotkeyStore";
import { CLIPBOARD_HOTKEYS, SIDEBAR_ICON_NAMES } from "./constants";
import { createAboutWindow, createSettingsWindow } from "./helpers";

export const parseShortcut = (hotkey: Hotkey) => {
  const { ctrl, alt, shift, key } = hotkey;
  const modifiers = [];
  if (ctrl) modifiers.push("CommandOrControl");
  if (alt) modifiers.push("Alt");
  if (shift) modifiers.push("Shift");
  return `${modifiers.join("+")}${
    modifiers.length ? "+" : ""
  }${key.toUpperCase()}`;
};

export let timer: NodeJS.Timeout | undefined;

export async function registerHotkeys(hotkeys: Hotkey[]) {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { getCurrentSidebarIcon, updateSidebarIcons } = AppStore;
  const { clipboards, clipboardRef } = ClipboardStore;
  await unregisterAll();

  // ############################################
  setGlobalHotkeyEvent(true);
  // ############################################

  // Display and hide the app window
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");
  if (mainHotkey?.status && !(await isRegistered(mainHotkey.shortcut))) {
    try {
      await register(mainHotkey.shortcut, () =>
        invoke("window_display_toggle")
      );
    } catch (_) {}
  }

  if (!await appWindow.isVisible()) return;

  // copy to clipboard
  try {
    await registerAll(CLIPBOARD_HOTKEYS, async (num) => {
      await invoke("copy_clipboard", { id: clipboards()[Number(num) - 1].id });
      removeAllHotkeyListeners();
    });
  } catch (_) {}

  // sidebar navigation
  const siderbarHotkeys = hotkeys.filter((h) =>
    SIDEBAR_ICON_NAMES.includes(h.name)
  );

  for (const hotkey of siderbarHotkeys) {
    try {
      if (hotkey.status)
        await register(hotkey.shortcut, () => updateSidebarIcons(hotkey.name));
    } catch (_) {}
  }

  const syncClipboardHistory = hotkeys.find(
    (h) => h.event === "sync_clipboard_history"
  );
  try {
    if (syncClipboardHistory?.status)
      await register(syncClipboardHistory.shortcut, () =>
        invoke("sync_clipboard_history")
      );
  } catch (error) {}

  const preferences = hotkeys.find((h) => h.event === "preferences");

  try {
    if (preferences?.status)
      await register(preferences.shortcut, createSettingsWindow);
  } catch (_) {}

  const about = hotkeys.find((h) => h.event === "about");

  try {
    if (about?.status) await register(about.shortcut, createAboutWindow);
  } catch (_) {}

  //exit
  const exitHotkey = hotkeys.find((h) => h.event === "exit");
  try {
    if (exitHotkey?.status)
      await register(exitHotkey.shortcut, async () => await exit(1));
  } catch (_) {}

  // scroll to top
  try {
    const scrollToTop = hotkeys.find((h) => h.event === "scroll_to_top");
    if (scrollToTop?.status && getCurrentSidebarIcon()?.name !== "View more") {
      await register(scrollToTop.shortcut, () =>
        clipboardRef()!.scrollTo(0, 0)
      );
    }
  } catch (_) {}

  timer = setTimeout(removeAllHotkeyListeners, 5000);
}

export const removeAllHotkeyListeners = async () => {
  const { hotkeys, setGlobalHotkeyEvent } = HotkeyStore;
  for (const key of CLIPBOARD_HOTKEYS) {
    try {
      await unregister(key);
    } catch (_) {}
  }

  for (const hotkey of hotkeys()) {
    if (hotkey.event === "window_display_toggle") continue;
    try {
      await unregister(hotkey.shortcut);
    } catch (_) {}
  }
  setGlobalHotkeyEvent(false);
  clearTimeout(timer);
};
