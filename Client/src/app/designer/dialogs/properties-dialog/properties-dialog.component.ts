// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { DiagramsService } from '../../../shared/services/diagrams.service';
import { ArrowDirection, DiagramData, LinkData, StatusDefinition, WeatherStatusDefinition } from '../../../shared/models/userobject.model'
import { Diagram } from '../../../shared/models/diagram.model';
import { Symbol, ButtonFunction } from '../../../shared/hmi.constants'
import { Router } from '@angular/router';
import { Subscription } from 'rxjs';
import { Hmi } from '../../../shared/hmi.constants'
import { getCommands, getCommandsByType } from '../../../shared/models/commands.model';

@Component({
  selector: 'app-properties-dialog',
  templateUrl: './properties-dialog.component.html',
  styleUrls: ['./properties-dialog.component.scss']
})
export class PropertiesDialogComponent implements OnInit {  
  label: string;
  name: string;
  displayDataLabel = "Reading/Status";
  controlDataLabel = "Control";
  visibilityDataLabel = "Visibility";
  mRID: string;
  fontSize: number;
  containerWidth: number;
  containerHeight: number;
  foreColor: any;
  backgroundColor: any;
  changeStyleAllowed: boolean;
  changeWidthAllowed: boolean;
  changeBackgroundAllowed: boolean;
  linkAllowed: boolean;
  deviceTypeMapping: string;
  displayData: any[];
  controlData: any[];
  visibilityData: any[];
  defaultFields = [];
  deviceTypeOptions = [];
  mRIdOptions: string[] = [];
  navigateToDataConnection: boolean = false;
  dataConnectAllowed: boolean;
  statusDefinitionAllowed: boolean = false;
  flowDefinitionAllowed: boolean = false; 
  weatherDefinitionAllowed: boolean = false; 
  equipmentList: any[];  
  selectedEquipment: any;
  textAlign: string;
  fontStyle: string;
  fontStyles: string[] = ['Normal', 'Bold', 'Italic', 'Bold+Italic'];
  textAlignAllowed: boolean = false; 
  fontSizeAllowed: boolean = true; 
  buttonFunction: string;
  buttonFunctionOptions: string[] = [ButtonFunction.link, ButtonFunction.command, ButtonFunction.setPoint];
  showLink: boolean = true;
  selectedCommand: string;  
  commandList: any[] = [];
  
  // For link
  selectedDiagramId: string;
  selectedLinkTarget: string;
  diagrams: Diagram[] = [];
  getItemSub: Subscription;
  getEquipmentSub: Subscription;
  getCommandSub: Subscription;
  linkTargetOptions: string[];

  // status definition
  statusDefinitions: StatusDefinition[];
  statusColors: string[] = ['gray', 'green', 'yellow', 'red'];
  isStatusDefinitionNumericDataType: boolean = false;

  // arrow definition
  arrowDirections: string[] = ['', 'east', 'west', 'south', 'north', 'se', 'sw', 'ne', 'nw'];
  arrowDirection: ArrowDirection;
  arrowColors: string[] = ['red', 'green'];

  // weather definition
  weatherDefinitions: WeatherStatusDefinition[];
  weatherConditions: string[] = ['clear', 'cloudy', 'rainy', 'storm', 'snow', 'partly-sunny', 'sunny'];

  constructor(
    public dialogRef: MatDialogRef<PropertiesDialogComponent>,
    private router: Router,
    private service: DiagramsService,
    @Inject(MAT_DIALOG_DATA) public data: DiagramData
  ) { 
    this.selectedEquipment = { name: '', mrid: ''};
    this.mRID = this.selectedEquipment.mrid = this.data.mRID; 
    this.getEquipmentList();               
  }

  ngOnInit() {            
    this.label = this.data.label;    
    this.name = this.data.name,    
    this.fontSize = this.data.fontSize;
    this.containerWidth = this.data.containerWidth;  
    this.containerHeight = this.data.containerHeight;  

    this.changeStyleAllowed = this.data.type === Symbol.measureBox 
      || this.data.type === Symbol.label 
      || this.data.type === Symbol.button 
      || this.data.type === Symbol.setPointButton
      || this.data.type === Symbol.statusIndicator 
      || this.data.type === Symbol.line
      || this.data.type === Symbol.curve;

    this.fontSizeAllowed = this.data.type !== Symbol.line && this.data.type !== Symbol.curve;
    
    this.deviceTypeMapping = this.data.deviceTypeMapping; 
    this.changeWidthAllowed = this.data.type === Symbol.measureBox;
    this.foreColor = this.data.foreColor;  
    this.backgroundColor = this.data.backgroundColor;      
    this.changeBackgroundAllowed = this.data.type === Symbol.measureBox || this.data.type === Symbol.button;    

    this.linkAllowed = this.data.type === Symbol.button;    
    this.dataConnectAllowed = Hmi.isDataConnectable(this.data.type);
    this.statusDefinitionAllowed = this.data.type === Symbol.statusIndicator;
    this.flowDefinitionAllowed = Hmi.isPowerFlow(this.data.type);
    this.weatherDefinitionAllowed = Hmi.isWeather(this.data.type);
    if (this.data.type === Symbol.label || this.data.type === Symbol.button || this.data.type === Symbol.statusIndicator) {
      this.textAlignAllowed = true;
      this.textAlign = this.data.textAlign || 'left';
      this.fontStyle = this.data.fontStyle || '0';
    }    

    // Button: link, command, set-point
    if (this.data.type == Symbol.button) {
      this.buttonFunction = this.data.func;
      if (!this.buttonFunction) {
        this.buttonFunction = ButtonFunction.link;
        this.showLink = true;
      }
      else {
        this.showLink = this.buttonFunction === ButtonFunction.link;
        this.getCommandList(this.buttonFunction);
      }
      this.selectedCommand = this.data.verb;
    }
    else {
      this.showLink = false;
    }            

    if (this.linkAllowed) {      
      this.linkTargetOptions = ['_blank', '_top',];

      if (this.data.linkData) {
        this.selectedDiagramId = this.data.linkData.diagramId;
        this.selectedLinkTarget = this.data.linkData.target;
      }

      this.getDiagrams();      
    }

    if (this.statusDefinitionAllowed) {
      if (this.data.statusDefinition) {
        this.statusDefinitions = this.data.statusDefinition;
      }
      else {
        this.statusDefinitions = [];
      }
    }

    if (this.weatherDefinitionAllowed) {
      if (this.data.weatherDefinition) {
        this.weatherDefinitions = this.data.weatherDefinition;
      }
      else {
        this.weatherDefinitions = [];
      }
    }
  
    if (this.data.displayData) {
      this.displayData = [...this.data.displayData];
      if (this.displayData.length > 0) {
        this.isStatusDefinitionNumericDataType = this.displayData[0].type === "analog";        
      }
    } 
    if (this.data.controlData) {
      this.controlData = [...this.data.controlData];
    } 

    if (this.data.visibilityData) {
      this.visibilityData = [...this.data.visibilityData];
    }

    if (this.flowDefinitionAllowed) {
      if (this.data.arrowDirection) {
        this.arrowDirection = this.data.arrowDirection;
      }
      else {
        this.arrowDirection= {
          positive: '',
          negative: '',
          neutral: ''
        }
      }
    }
  }

  getDiagrams() {
    this.getItemSub = this.service.getAll()
      .subscribe(data => {
        this.diagrams = data;        
      });
  }

  getEquipmentList() {
    this.getEquipmentSub = this.service.getEquipmentList()
      .subscribe(data => {
        this.equipmentList = data; 
        this.equipmentList.unshift({name: '', mrid: ''});       
        if (this.equipmentList)
        {
          for(var i = 0; i < this.equipmentList.length; ++i)
          {            
            if (this.equipmentList[i].mrid === this.mRID) {
              this.selectedEquipment = this.equipmentList[i];
              break;
            }
          }
        }
      });
  }

  getCommandList(type: String) {    
    this.commandList = [];      
    let commandTypes = getCommands();
    for (let entry of commandTypes) {
      let a = getCommandsByType(entry);

      for (let cmd of a) {        
        if (cmd.attributes.type === type) {          
          this.commandList.push(cmd);
        }
      }      
    }    
  }

  // save all grid item data
  onSave(): void {
    console.log(this.backgroundColor);

    var foreC = null;
    if (this.foreColor) {
      if (this.foreColor.hex) {
        foreC = '#' + this.foreColor.hex;
      }
      else {
        var temp = '' + this.foreColor;
        if (temp.startsWith('#')) {
          foreC = temp;
        }
      }
    }

    var backgroundC = null;
    if (this.backgroundColor) {
      if (this.backgroundColor.hex) {
        backgroundC = '#' + this.backgroundColor.hex;
      }
      else {
        var temp = '' + this.backgroundColor;
        if (temp.startsWith('#')) {
          backgroundC = temp;
        }
      }
    } 

    var linkData: LinkData = null;
    
    if (this.linkAllowed) {
      linkData = {
        diagramId: this.selectedDiagramId,
        target: this.selectedLinkTarget
      };
    }

    this.dialogRef.close({
      label: this.label,
      name: this.selectedEquipment?.name || this.name,
      displayData: this.displayData,
      controlData: this.controlData,
      visibilityData: this.visibilityData,
      mRID: this.selectedEquipment?.mrid,
      fontSize: this.fontSize,
      fontStyle: this.fontStyle,
      textAlign: this.textAlign,
      containerWidth: this.containerWidth,
      containerHeight: this.containerHeight,
      foreColor: foreC,
      backgroundColor: backgroundC,
      linkData: linkData,
      deviceTypeMapping: this.deviceTypeMapping,
      navigateToDataConnection: this.navigateToDataConnection,
      statusDefinition: this.statusDefinitions,
      weatherDefinition: this.weatherDefinitions,
      arrowDirection: this.arrowDirection,
      selectedCommand: this.selectedCommand,
      func: this.buttonFunction
    });
  }

  // close modal window
  onNoClick(): void {
    this.dialogRef.close();
  }  

  setSelectedFields(allFields: any[], selectedFields: any[]): any[] {
    if (selectedFields.length) {
     return allFields.map( elem => {
       selectedFields.forEach(selectElem => {
         if (selectElem.value === elem.value) {
           elem.selected = true;
         }
       });
        return elem;
      });
    }
    return allFields;
  }    
  
  // navigate to data connect screen
  dataConnect() { 
    this.navigateToDataConnection = true;   
    this.onSave();    
  }

  addStatusDefinition() {
    const def: StatusDefinition = {
      value: 0,      
      color: "gray"
    };
    this.statusDefinitions.push(def);
  }

  removeStatusDefinition(item: StatusDefinition) {
    if (item) {
      for(let index = this.statusDefinitions.length - 1; index >=0; --index) {
        var obj = this.statusDefinitions[index];
        if (obj === item) {        
          this.statusDefinitions.splice(index, 1);
        }
      }
    }
    else {
      console.error("Unable to delete status definition.  item=" + item);
    }
  }

  addWeatherDefinition() {
    const def: WeatherStatusDefinition = {
      from: 0,
      to: 0,
      text: 'clear'
    };
    this.weatherDefinitions.push(def);
  }

  removeWeatherDefinition(item: WeatherStatusDefinition) {
    if (item) {
      for(let index = this.weatherDefinitions.length - 1; index >=0; --index) {
        var obj = this.weatherDefinitions[index];
        if (obj === item) {        
          this.weatherDefinitions.splice(index, 1);
        }
      }
    }
    else {
      console.error("Unable to delete weather definition.  item=" + item);
    }
  }

  buttonFunctionChanged(event: any) {
    this.showLink = event.value === ButtonFunction.link;
    this.buttonFunction = event.value;
    if (this.buttonFunction === ButtonFunction.command) {
      this.getCommandList(ButtonFunction.command);
      this.selectedCommand = "";
    }
    else if (this.buttonFunction === ButtonFunction.setPoint) {
      this.getCommandList(ButtonFunction.setPoint);
      this.selectedCommand = "";
    }
  }
}
