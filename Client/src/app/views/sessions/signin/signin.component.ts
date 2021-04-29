// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, ViewChild, OnDestroy, AfterViewInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { MatButton } from '@angular/material/button';
import { MatProgressBar } from '@angular/material/progress-bar';
import { Validators, FormGroup, FormControl } from '@angular/forms';
import { Subject } from 'rxjs';
import { AppLoaderService } from '../../../shared/services/app-loader/app-loader.service';
import { JwtAuthService } from '../../../shared/services/auth/jwt-auth.service';

@Component({
  selector: 'app-signin',
  templateUrl: './signin.component.html',
  styleUrls: ['./signin.component.scss']
})
export class SigninComponent implements OnInit, AfterViewInit, OnDestroy {
  @ViewChild(MatProgressBar) progressBar: MatProgressBar;
  @ViewChild(MatButton) submitButton: MatButton;

  signinForm: FormGroup;
  errorMsg = '';
  
  private _unsubscribeAll: Subject<any>;

  constructor(
    private jwtAuth: JwtAuthService,
    private loader: AppLoaderService,
    private router: Router,
    private route: ActivatedRoute
  ) {
    this._unsubscribeAll = new Subject();
  }

  ngOnInit() {
    this.signinForm = new FormGroup({
      username: new FormControl('', Validators.required),
      password: new FormControl('', Validators.required)      
    });
  }

  ngAfterViewInit() {
    this.autoSignIn();
  }

  ngOnDestroy() {
    this._unsubscribeAll.next();
    this._unsubscribeAll.complete();
  }

  signin() {
    const signinData = this.signinForm.value

    this.submitButton.disabled = true;
    this.progressBar.mode = 'indeterminate';  
    
    this.errorMsg = '';
    
    this.jwtAuth.signin(signinData.username, signinData.password)
    .subscribe(response => {
      this.router.navigateByUrl(this.jwtAuth.return);
    }, err => {
      this.submitButton.disabled = false;
      this.progressBar.mode = 'determinate';
      this.errorMsg = "Invalid username or password";
      console.log(err);
    })
  }

  autoSignIn() {    
    if(this.jwtAuth.return === '/') {
      return
    }
    this.loader.open(`Logging in! \n Return url: ${this.jwtAuth.return.substring(0, 20)}...`, {width: '320px'});
    setTimeout(() => {
      this.signin();      
      this.loader.close()
    }, 2000);
  }

}
