// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { UntypedFormBuilder, Validators, UntypedFormGroup } from '@angular/forms';
import { getEquipmentTypeList } from '../../../shared/models/equipment.model';
import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-devicedialogs',
  templateUrl: './devicedialogs.component.html',
  styleUrls: ['./devicedialogs.component.scss']
})
export class DeviceDialogsComponent implements OnInit {  
  public itemForm: UntypedFormGroup; 
  deviceTypes: any[];   
  canEditMrid: boolean = false;
  constructor(
    @Inject(MAT_DIALOG_DATA) public data: any,
    public dialogRef: MatDialogRef<DeviceDialogsComponent>,
    private fb: UntypedFormBuilder,
  ) {}

  ngOnInit() { 
    this.deviceTypes = getEquipmentTypeList(); 
    this.canEditMrid = this.data.isNew;
    this.buildItemForm(this.data.payload);        
  }
  buildItemForm(item) { 
    var disabled : boolean = this.canEditMrid ? false : true;       
    this.itemForm = this.fb.group({
      mrid: [{ value: item.mrid || '', disabled: disabled }, Validators.compose([Validators.required, Validators.pattern('^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$')])],
      name: [item.name || '', Validators.required],
      deviceType: [item.deviceType || '', Validators.required]     
    });         
  }

  idGenerator() {     
    this.itemForm.controls['mrid'].setValue(uuidv4());    
  }; 

  submit() {    
    this.dialogRef.close({
      name: this.itemForm.value.name,
      deviceType: this.itemForm.value.deviceType,
      mrid: this.itemForm.controls['mrid'].value,
    })
  }
}
