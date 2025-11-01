const DARK_CLASS = "dark"
const COLOR_SCHEME_ATTRIBUTE = "data-color-scheme"
const COLOR_SCHEME_DARK = "dark"
const COLOR_SCHEME_LIGHT = "light"

const THEME_VALUES = ["light", "dark", "system"] as const

export type ThemeMode = (typeof THEME_VALUES)[number]

type MediaQueryChangeListener = (event: MediaQueryListEvent) => void

type MediaQueryListWithLegacy = MediaQueryList & {
  addListener?: (listener: MediaQueryChangeListener) => void
  removeListener?: (listener: MediaQueryChangeListener) => void
}

let systemMediaQuery: MediaQueryList | null = null
let systemMediaQueryListener: MediaQueryChangeListener | null = null

function isClientEnvironment(): boolean {
  return typeof window !== "undefined" && typeof document !== "undefined"
}

function supportsMatchMedia(): boolean {
  return isClientEnvironment() && typeof window.matchMedia === "function"
}

function setDocumentTheme(isDark: boolean) {
  if (!isClientEnvironment()) {
    return
  }

  const root = document.documentElement

  if (isDark) {
    root.classList.add(DARK_CLASS)
    root.setAttribute(COLOR_SCHEME_ATTRIBUTE, COLOR_SCHEME_DARK)
  } else {
    root.classList.remove(DARK_CLASS)
    root.setAttribute(COLOR_SCHEME_ATTRIBUTE, COLOR_SCHEME_LIGHT)
  }
}

function cleanupSystemMediaQuery() {
  if (!systemMediaQuery) {
    return
  }

  if (systemMediaQueryListener) {
    systemMediaQuery.removeEventListener("change", systemMediaQueryListener)
    const legacyMediaQuery = systemMediaQuery as MediaQueryListWithLegacy
    const legacyRemoveListener = legacyMediaQuery.removeListener
    if (typeof legacyRemoveListener === "function") {
      legacyRemoveListener.call(legacyMediaQuery, systemMediaQueryListener)
    }
  }

  systemMediaQuery = null
  systemMediaQueryListener = null
}

function ensureSystemMediaQuery(): MediaQueryList | null {
  if (!supportsMatchMedia()) {
    return null
  }

  if (!systemMediaQuery) {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
    const listener = (event: MediaQueryListEvent) => {
      setDocumentTheme(event.matches)
    }

    mediaQuery.addEventListener("change", listener)
    const legacyMediaQuery = mediaQuery as MediaQueryListWithLegacy
    const legacyAddListener = legacyMediaQuery.addListener
    if (typeof legacyAddListener === "function") {
      legacyAddListener.call(legacyMediaQuery, listener)
    }

    systemMediaQuery = mediaQuery
    systemMediaQueryListener = listener
  }

  return systemMediaQuery
}

function applySystemTheme() {
  const mediaQuery = ensureSystemMediaQuery()
  if (!mediaQuery) {
    setDocumentTheme(false)
    return
  }

  setDocumentTheme(mediaQuery.matches)
}

export function applyTheme(theme: ThemeMode) {
  if (!isClientEnvironment()) {
    return
  }

  cleanupSystemMediaQuery()

  if (!THEME_VALUES.includes(theme)) {
    setDocumentTheme(false)
    return
  }

  switch (theme) {
    case "system":
      applySystemTheme()
      break
    case "dark":
      setDocumentTheme(true)
      break
    default:
      setDocumentTheme(false)
      break
  }
}

export function disposeThemeListener() {
  cleanupSystemMediaQuery()
}
