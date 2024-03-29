// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, OnDestroy } from '@angular/core';
import { UserService } from 'src/app/shared/services/users.service';
import { Subscription } from 'rxjs';
import { MatDialogRef, MatDialog } from '@angular/material/dialog';
import { MatSnackBar } from '@angular/material/snack-bar';
import { DialogsComponent } from './dialogs/dialogs.component';
import { AppLoaderService } from '../../shared/services/app-loader/app-loader.service';
import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.scss']
})
export class UsersComponent implements OnInit, OnDestroy {
  public rows = [];
  columns = [];
  temp = [];
  public getItemSub: Subscription;

  constructor(
    private service: UserService,
    private dialog: MatDialog,
    private snack: MatSnackBar,
    private loader: AppLoaderService
  ) { }

  ngOnInit(): void {    
    this.getData();
  }

  ngOnDestroy() {
    if (this.getItemSub) {
      this.getItemSub.unsubscribe()
    }
  }

  getData() {
    this.getItemSub = this.service.getAll()
      .subscribe(data => {
        this.rows = this.temp = data;
      })
  }

  updateFilter(event) {
    const val = event.target.value.toLowerCase();
    var columns = Object.keys(this.temp[0]);
    // Removes last "$$index" from "column"
    columns.splice(columns.length - 1);

    // console.log(columns);
    if (!columns.length)
      return;

    const rows = this.temp.filter(function(d) {
      for (let i = 0; i <= columns.length; i++) {
        let column = columns[i];
        // console.log(d[column]);
        if (d[column] && d[column].toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
      }
    });

    this.rows = rows;
  }
  
  delete(id: string) {
    console.log("delete user: " + id);

    if(confirm("Are you sure to delete user ID="+id)) {
      this.service.delete(id).subscribe(
        data => {
          this.rows = data;
          this.loader.close();
          this.snack.open('User Deleted!', 'OK', { duration: 4000 })
        },
        error => {
          this.loader.close();
          this.snack.open('Unable to delete user!', 'OK', { duration: 4000 });                
        }
      );
    }
  }

  addOrEdit(data: any = {}, isNew?) {
    let title = isNew ? 'Add new user' : 'Update user';
    let dialogRef: MatDialogRef<any> = this.dialog.open(DialogsComponent, {
      width: '720px',
      disableClose: true,
      data: { title: title, payload: data, isNew: isNew ? isNew : false }
    })
    dialogRef.afterClosed()
      .subscribe(res => {
        if(!res) {
          // If user press cancel
          return;
        }
        this.loader.open();
        if (isNew) {
          res.id = uuidv4();
          this.service.create(res)
            .subscribe(
              data => {
                this.rows = data;
                this.loader.close();
                this.snack.open('User Added!', 'OK', { duration: 4000 });              
              },
              error => {
                this.loader.close();
                this.snack.open('Unable to add user!', 'OK', { duration: 4000 });                
              }
            )
        } else {          
          this.service.update(res)
            .subscribe(
              data => {
                this.rows = data;
                this.loader.close();
                this.snack.open('User Updated!', 'OK', { duration: 4000 })
              },
              error => {
                this.loader.close();
                this.snack.open('Unable to update user!', 'OK', { duration: 4000 });                
              }
            )
        }
      })
  }

}
