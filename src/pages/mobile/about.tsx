import MobileAppLayout from "@/components/layout/mobile/MobileAppLayout";
import React from "react";
import AboutPage from "../desktop/about";

export default function about() {
  return (
    <MobileAppLayout>
      <AboutPage />
    </MobileAppLayout>
  );
}
