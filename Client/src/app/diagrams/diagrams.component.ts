// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, OnDestroy, Renderer2 } from '@angular/core';
import { DiagramsService } from '../shared/services/diagrams.service';
import { Router } from '@angular/router';

import { MatDialogRef, MatDialog } from '@angular/material/dialog';
import { MatSnackBar } from '@angular/material/snack-bar';
import { DialogsComponent } from './dialogs/dialogs.component';
import { Subscription } from 'rxjs';
import { AppLoaderService } from '../shared/services/app-loader/app-loader.service';
import { Diagram } from '../shared/models/diagram.model';
import { JwtAuthService } from "../shared/services/auth/jwt-auth.service";
import { v4 as uuidv4 } from 'uuid';
import { Authorization } from '../shared/models/user.model';


@Component({
  selector: 'app-diagrams',
  templateUrl: './diagrams.component.html',
  styleUrls: ['./diagrams.component.scss']
})
export class DiagramsComponent implements OnInit, OnDestroy {
  public rows = [];
  temp = [];
  canEditDiagram: boolean = false;
  public getItemSub: Subscription;

  constructor(
    private renderer: Renderer2,
    private service: DiagramsService, 
    private router: Router,
    private dialog: MatDialog,
    private snack: MatSnackBar,
    private jwtAuth: JwtAuthService,
    private loader: AppLoaderService) { }

  ngOnInit() {   
    this.canEditDiagram = Authorization.canEditDiagram( this.jwtAuth.getUserRole());
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
      },
      error => {
        console.error(error);
        this.rows = this.temp = [];
        this.snack.open(error, 'OK', { duration: 4000 });
      });
  }

  updateFilter(event) {
    const val = event.target.value.toLowerCase();
    var columns = Object.keys(this.temp[0]);
    // Removes last "$$index" from "column"
    columns.splice(columns.length - 1);
    
    if (!columns.length)
      return;

    const rows = this.temp.filter(function(d) {
      for (let i = 0; i <= columns.length; i++) {
        let column = columns[i];       
        if (d[column] && d[column].toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
      }
    });

    this.rows = rows;
  }

  open(id: string) {         
    this.router.navigateByUrl('/designer?id=' + id);
  }

  run(id: string) {
    console.log("run diagram: " + id); 
    this.router.navigateByUrl('/hmi?id=' + id);
  }

  export(id: string) {
    this.service.get(id).subscribe(
      data => {               
        try {
          const blob = new Blob([JSON.stringify(data)], { type: 'text/json;charset=utf-8' });
          const url = window.URL;
          const link = url.createObjectURL(blob);
          const downloadLink = this.renderer.createElement('a');
          this.renderer.setStyle(downloadLink, 'display', 'none');
          this.renderer.setAttribute(downloadLink, 'download', data.diagramId + '.json');
          this.renderer.setAttribute(downloadLink, 'href', link);
          downloadLink.click();
        }
        catch (e) {
          console.error(e);
        }
      },
      error => {
        console.error(error);
        this.snack.open(error, 'OK', { duration: 4000 });
      }
    ); 
  }

  delete(row: any) {
    console.log("delete diagram: " + row.diagramId);

    if(confirm("Are you sure to delete diagram name "+row.name)) {           
      this.service.delete(row.diagramId)
          .subscribe(data => {                              
            this.snack.open('Diagram deleted!', 'OK', { duration: 4000 })
          }, error => {
            console.error(error);
            this.snack.open(error, 'OK', { duration: 4000 });
          });

      this.getData();
    }
  }

  connect(id: string) {
    console.log("data connect: " + id);
    this.router.navigateByUrl('/data-connect?id=' + id);
  }

  addOrEdit(data: any = {}, isNew?) {
    let title = isNew ? 'Add new diagram' : 'Update diagram';
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

        const diagram: Diagram = {
          diagramId: res.diagramId ? res.diagramId : uuidv4(),
          name: res.name,
          description: res.description || '',
          location: res.location || '',
          backgroundColor: res.backgroundColor && res.backgroundColor.hex ? '#' + res.backgroundColor.hex : '',
          data: data.data || '', // graph data
          createdBy: res.createdBy || '',
          createdDate: res.createdDate? res.createdDate : new Date().toLocaleDateString()
        }
        console.log(diagram);
        this.loader.open();
        try {
          if (isNew) {
            this.service.create(diagram)
              .subscribe(data => {                              
                this.snack.open('Diagram Added!', 'OK', { duration: 4000 });                
                this.getData();
              }, error => {
                console.error(error);
                this.snack.open(error, 'OK', { duration: 4000 });
              });
          } else {
            this.service.update(diagram)
              .subscribe(data => {                              
                this.snack.open('Diagram Updated!', 'OK', { duration: 4000 })
                this.getData();
              }, error => {
                console.error(error);
                this.snack.open(error, 'OK', { duration: 4000 });
              });
          }
        }
        catch (e) {
          this.snack.open(e, 'OK', { duration: 4000 });
        }
        finally {
          this.loader.close();
        }
      })
  }
}
