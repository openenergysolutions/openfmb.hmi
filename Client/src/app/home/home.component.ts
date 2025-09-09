// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, inject } from "@angular/core";
import { JwtAuthService } from "../shared/services/auth/jwt-auth.service";
import { Authorization } from "../shared/models/user.model";

@Component({
    selector: "app-home",
    templateUrl: "./home.component.html",
    styleUrls: ["./home.component.scss"]
})
export class HomeComponent implements OnInit {
  private jwtService = inject(JwtAuthService);

  canEditDiagram: boolean = false;

  ngOnInit(): void {
    this.canEditDiagram = Authorization.canEditDiagram(
      this.jwtService.getUserRole(),
    );
  }
}
