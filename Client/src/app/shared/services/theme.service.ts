// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, Renderer2, RendererFactory2, EventEmitter, inject } from "@angular/core";
import { DOCUMENT } from "@angular/common";

export interface ITheme {
  name: string;
  baseColor?: string;
  isActive?: boolean;
}

@Injectable()
export class ThemeService {
  private document = inject<Document>(DOCUMENT);

  public onThemeChange: EventEmitter<ITheme> = new EventEmitter();

  public mainThemes: ITheme[] = [
    {
      name: "hmi-navy",
      baseColor: "#10174c",
      isActive: false,
    },
  ];
  public activatedTheme: ITheme;
  private renderer: Renderer2;
  constructor() {
    const rendererFactory = inject(RendererFactory2);

    this.renderer = rendererFactory.createRenderer(null, null);
  }

  // Invoked in AppComponent and apply 'activatedTheme' on startup
  applyMatTheme(themeName: string) {
    this.activatedTheme = this.mainThemes.find((t) => t.name === themeName);
    this.flipActiveFlag(themeName);

    // this.changeTheme(themeName);
    this.renderer.addClass(this.document.body, themeName);
  }

  changeTheme(prevTheme, themeName: string) {
    this.renderer.removeClass(this.document.body, prevTheme);
    this.renderer.addClass(this.document.body, themeName);
    this.flipActiveFlag(themeName);
    this.onThemeChange.emit(this.activatedTheme);
  }

  flipActiveFlag(themeName: string) {
    this.mainThemes.forEach((t) => {
      t.isActive = false;
      if (t.name === themeName) {
        t.isActive = true;
        this.activatedTheme = t;
      }
    });
  }
}
