// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, ViewChild, OnDestroy, AfterViewInit, inject } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { MatButton } from "@angular/material/button";
import { MatProgressBar } from "@angular/material/progress-bar";
import {
  Validators,
  UntypedFormGroup,
  UntypedFormControl,
} from "@angular/forms";
import { Subject } from "rxjs";
import { AppLoaderService } from "../../../shared/services/app-loader/app-loader.service";
import { JwtAuthService } from "../../../shared/services/auth/jwt-auth.service";

@Component({
    selector: "app-signin",
    templateUrl: "./signin.component.html",
    styleUrls: ["./signin.component.scss"]
})
export class SigninComponent implements OnInit, AfterViewInit, OnDestroy {
  private jwtAuth = inject(JwtAuthService);
  private loader = inject(AppLoaderService);
  private router = inject(Router);
  private route = inject(ActivatedRoute);

  @ViewChild(MatProgressBar) progressBar: MatProgressBar;
  @ViewChild(MatButton) submitButton: MatButton;

  signinForm: UntypedFormGroup;
  errorMsg = "";

  private _unsubscribeAll: Subject<any>;

  constructor() {
    this._unsubscribeAll = new Subject();
  }

  ngOnInit() {
    this.signinForm = new UntypedFormGroup({
      username: new UntypedFormControl("", Validators.required),
      password: new UntypedFormControl("", Validators.required),
    });
  }

  ngAfterViewInit() {
    this.autoSignIn();
  }

  ngOnDestroy() {
    this._unsubscribeAll.complete();
  }

  signin() {
    const signinData = this.signinForm.value;

    this.submitButton.disabled = true;
    this.progressBar.mode = "indeterminate";

    this.errorMsg = "";

    this.jwtAuth.signin(signinData.username, signinData.password).subscribe(
      () => {
        this.router.navigateByUrl(this.jwtAuth.return);
      },
      (err) => {
        this.submitButton.disabled = false;
        this.progressBar.mode = "determinate";
        this.errorMsg = "Invalid username or password";
        console.log(err);
      },
    );
  }

  autoSignIn() {
    if (this.jwtAuth.return === "/") {
      return;
    }
    this.loader.open(
      `Logging in! \n Return url: ${this.jwtAuth.return.substring(0, 20)}...`,
      { width: "320px" },
    );
    setTimeout(() => {
      this.signin();
      this.loader.close();
    }, 2000);
  }
}
