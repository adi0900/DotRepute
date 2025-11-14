import { LunokitTheme } from "@luno-kit/ui";

export const lightTheme: LunokitTheme = {
  colors: {
    accentColor: "#000000",
    success: "#10b981",
    successForeground: "#ffffff",
    warning: "#f59e0b",
    warningForeground: "#1f2937",
    error: "#ef4444",
    errorForeground: "#ffffff",
    info: "#3b82f6",
    infoForeground: "#ffffff",

    // Connect Button
    connectButtonBackground: "#000000",
    connectButtonInnerBackground: "#f5f5f5",
    connectButtonText: "#ffffff",

    // Wallet Select
    walletSelectItemBackground: "#ffffff",
    walletSelectItemBackgroundHover: "#f5f5f5",
    walletSelectItemText: "#ffffff",

    // Account Action
    accountActionItemBackground: "#ffffff",
    accountActionItemBackgroundHover: "#f5f5f5",
    accountActionItemText: "#000000",

    // Account Select
    accountSelectItemBackground: "#ffffff",
    accountSelectItemBackgroundHover: "#f5f5f5",
    accountSelectItemText: "#ffffff",

    // Network Button
    currentNetworkButtonBackground: "#ffffff",
    currentNetworkButtonText: "#000000",

    // Network Select
    networkSelectItemBackground: "#ffffff",
    networkSelectItemBackgroundHover: "#f5f5f5",
    networkSelectItemText: "#ffffff",

    // Asset Select
    assetSelectItemBackground: "#ffffff",

    // Navigation
    navigationButtonBackground: "#ffffff",

    // Modal
    modalBackground: "#ffffff",
    modalBackdrop: "rgba(0, 0, 0, 0.5)",
    modalBorder: "#333333",
    modalText: "#000000",
    modalTextSecondary: "#333333",
    modalControlButtonBackgroundHover: "#f5f5f5",
    modalControlButtonText: "#000000",

    // UI Elements
    separatorLine: "#333333",
    defaultIconBackground: "#ffffff",
    skeleton: "#f5f5f5",
  },
  fonts: {
    body: "Inter, sans-serif",
  },
  radii: {
    walletSelectItem: "1px",
    connectButton: "1px",
    modalControlButton: "1px",
    accountActionItem: "1px",
    accountSelectItem: "1px",
    currentNetworkButton: "1px",
    networkSelectItem: "1px",
    assetSelectItem: "1px",
    modal: "2px",
    modalMobile: "2px",
  },
  shadows: {
    button: "0 1px 3px rgba(0, 0, 0, 0.1)",
    modal:
      "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
  },
  blurs: {
    modalOverlay: "8px",
  },
};

export const darkTheme: LunokitTheme = {
  colors: {
    accentColor: "#ffffff",
    success: "#ffffff",
    successForeground: "#000000",
    warning: "#ffffff",
    warningForeground: "#000000",
    error: "#ffffff",
    errorForeground: "#000000",
    info: "#ffffff",
    infoForeground: "#000000",

    // Connect Button
    connectButtonBackground: "#ffffff",
    connectButtonInnerBackground: "#ffffff",
    connectButtonText: "#000000",

    // Wallet Select
    walletSelectItemBackground: "#000000",
    walletSelectItemBackgroundHover: "#1a1a1a",
    walletSelectItemText: "#ffffff",

    // Account Action
    accountActionItemBackground: "#000000",
    accountActionItemBackgroundHover: "#1a1a1a",
    accountActionItemText: "#ffffff",

    // Account Select
    accountSelectItemBackground: "#000000",
    accountSelectItemBackgroundHover: "#1a1a1a",
    accountSelectItemText: "#ffffff",

    // Network Button
    currentNetworkButtonBackground: "#000000",
    currentNetworkButtonText: "#ffffff",

    // Network Select
    networkSelectItemBackground: "#000000",
    networkSelectItemBackgroundHover: "#1a1a1a",
    networkSelectItemText: "#ffffff",

    // Asset Select
    assetSelectItemBackground: "#000000",

    // Navigation
    navigationButtonBackground: "#000000",

    // Modal
    modalBackground: "#000000",
    modalBackdrop: "rgba(0, 0, 0, 0.8)",
    modalBorder: "#ffffff",
    modalText: "#ffffff",
    modalTextSecondary: "#cccccc",
    modalControlButtonBackgroundHover: "#1a1a1a",
    modalControlButtonText: "#ffffff",

    // UI Elements
    separatorLine: "#cccccc",
    defaultIconBackground: "#000000",
    skeleton: "#1a1a1a",
  },
  fonts: {
    body: "Inter, sans-serif",
  },
  radii: {
    walletSelectItem: "1px",
    connectButton: "1px",
    modalControlButton: "1px",
    accountActionItem: "1px",
    accountSelectItem: "1px",
    currentNetworkButton: "1px",
    networkSelectItem: "1px",
    assetSelectItem: "1px",
    modal: "2px",
    modalMobile: "2px",
  },
  shadows: {
    button: "0 1px 3px rgba(0, 0, 0, 0.3)",
    modal:
      "0 20px 25px -5px rgba(0, 0, 0, 0.3), 0 10px 10px -5px rgba(0, 0, 0, 0.2)",
  },
  blurs: {
    modalOverlay: "8px",
  },
};
