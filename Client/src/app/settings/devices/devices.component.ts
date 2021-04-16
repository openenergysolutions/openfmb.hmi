import { Component, OnInit, OnDestroy } from '@angular/core';
import { DiagramsService } from 'src/app/shared/services/diagrams.service';
import { Subscription } from 'rxjs';
import { MatDialogRef, MatDialog } from '@angular/material/dialog';
import { MatSnackBar } from '@angular/material/snack-bar';
import { DeviceDialogsComponent } from './dialogs/devicedialogs.component';
import { AppLoaderService } from '../../shared/services/app-loader/app-loader.service';
import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-devices',
  templateUrl: './devices.component.html',
  styleUrls: ['./devices.component.scss']
})
export class DevicesComponent implements OnInit, OnDestroy {
  public rows = [];
  columns = [];
  temp = [];
  public getItemSub: Subscription;

  constructor(
    private service: DiagramsService,
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
    this.getItemSub = this.service.getEquipmentList()
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
    console.log("delete device: " + id);

    if(confirm("Are you sure to delete device with MRID="+id)) {
      this.service.deleteEquipment(id).subscribe(
        data => {
          this.rows = data;
          this.loader.close();
          this.snack.open('Device Deleted!', 'OK', { duration: 4000 })
        },
        error => {
          this.loader.close();
          this.snack.open('Unable to delete device!', 'OK', { duration: 4000 });                
        }
      );
    }
  }

  addOrEdit(data: any = {}, isNew?) {
    let title = isNew ? 'Add new device' : 'Update device';
    let dialogRef: MatDialogRef<any> = this.dialog.open(DeviceDialogsComponent, {
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
          this.service.createEquipment(res)
            .subscribe(
              data => {
                this.rows = data;
                this.loader.close();
                this.snack.open('Device Added!', 'OK', { duration: 4000 });              
              },
              error => {
                this.loader.close();
                this.snack.open('Unable to add device!', 'OK', { duration: 4000 });                
              }
            )
        } else {          
          this.service.updateEquipment(res)
            .subscribe(
              data => {
                this.rows = data;
                this.loader.close();
                this.snack.open('Device Updated!', 'OK', { duration: 4000 })
              },
              error => {
                this.loader.close();
                this.snack.open('Unable to update device!', 'OK', { duration: 4000 });                
              }
            )
        }
      })
  }

}
