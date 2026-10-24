import React from "react";
import markUrl from "../../assets/brand/vozprompt-mark.png";
const BRAND_NAME = "VozPrompt";

const HandyTextLogo = ({
  width = 160,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => (
  <div
    role="img"
    aria-label="VozPrompt"
    className={`vozprompt-logo ${className ?? ""}`}
    style={{
      display: "inline-flex",
      alignItems: "center",
      gap: "0.3em",
      width,
      height,
      maxWidth: "100%",
      fontSize: width * 0.13,
      lineHeight: 1,
      color: "var(--color-text)",
    }}
  >
    <i
      aria-hidden="true"
      className="brand-mark"
      style={{
        display: "block",
        flex: "0 0 1.8em",
        height: "1.8em",
        background: "var(--color-logo-primary)",
        mask: `url(${markUrl}) center / contain no-repeat`,
        WebkitMask: `url(${markUrl}) center / contain no-repeat`,
      }}
    />
    <b
      className="brand-wordmark"
      style={{ fontWeight: 650, letterSpacing: "-0.055em" }}
    >
      {BRAND_NAME}
    </b>
  </div>
);
export default HandyTextLogo;
