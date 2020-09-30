import { Component, OnInit } from '@angular/core';
import {CdkDragDrop, moveItemInArray, transferArrayItem} from '@angular/cdk/drag-drop';
import {FormControl} from '@angular/forms';
import { DiagramsService } from '../shared/services/diagrams.service';
import { Diagram } from '../shared/models/diagram.model';
import { Subscription } from 'rxjs';
import * as converter from 'xml-js';
import { getProfilesByModule, getModules } from '../shared/models/openfmb.model';
import { MatSnackBar } from '@angular/material/snack-bar';
import { ActivatedRoute } from '@angular/router'

@Component({
  selector: 'app-data-connect',
  templateUrl: './data-connect.component.html',
  styleUrls: ['./data-connect.component.scss']
})
export class DataConnectComponent implements OnInit {
  selectDiagramControl = new FormControl();
  diagrams: Diagram[] = [];
  graphItems: any[] = [];
  modules: any[] = [];
  profiles: any[] = [];
  dataPoints: any[] = [];
  graphModelData: any;

  // Filter
  filter: "";
  temp = [];

  getItemSub: Subscription;
  selectedDiagramId: string;
  requestDiagramId: string;
  requestCellId: string;
  selectedGraphItemId: string;
  selectedGraphItem: any;
  selectedModule: string;
  selectedProfile: string;

  diagram: Diagram;
  graphModel : any;

  availablePoints = [];   
  currentPoints = [];

  constructor(
    private service: DiagramsService,
    private snack: MatSnackBar,
    private activateRoute : ActivatedRoute) {
      this.activateRoute.queryParams.subscribe(params => {
        this.requestDiagramId = params['id'];              
        this.requestCellId = params['cell'];

        console.log("data-connect:: requested cell id = " + this.requestCellId);
      });
    }

  ngOnInit(): void {
    this.getDiagrams();
    this.modules = getModules();
  }

  drop(event: CdkDragDrop<string[]>) {    
    if (event.previousContainer === event.container) {
      moveItemInArray(event.container.data, event.previousIndex, event.currentIndex);
    } else {
      if (this.selectedDiagramId && this.selectedGraphItemId) {
        if (!event.container.data) {
          event.container.data = [];
        }

        if (this.selectedGraphItem.type !== 'measure-box' && event.container.data.length > 0) {
          this.snack.open('Only one mapping is allowed.', 'OK', { duration: 4000 });
        }
        else {
          transferArrayItem(event.previousContainer.data,
                          event.container.data,
                          event.previousIndex,
                          event.currentIndex);
        }
      }
      else {
        this.snack.open('Please select diagram and graph item.', 'OK', { duration: 4000 })
      }
    }
  }

  getDiagrams() {
    this.getItemSub = this.service.getAll()
      .subscribe(data => {
        this.diagrams = data;
        if (this.requestDiagramId) {
          this.selectedDiagramId = this.requestDiagramId;
          this.onDiagramChanged(this.selectedDiagramId);
        }         
      })
  }

  onDiagramChanged(id: string) {
    console.log("data-connect:: on diagram changed: " + id);
    this.selectedDiagramId = id;    
    this.service.get(id).subscribe(data => {
      this.diagram = data;

      if (!this.diagram) {
        this.snack.open('Unable to retrieve digram with ID ' + id, 'OK', { duration: 4000 });
        return;
      }
  
      var xml = this.diagram.data;
      
      this.graphModel = converter.xml2js(xml);        
  
      var elem = this.graphModel.elements[0].elements[0];
      
      this.graphItems = [];
      this.selectedGraphItemId = null;
      this.currentPoints = [];
  
      for(var i = 0; i < elem.elements.length; ++i) {
        var e = elem.elements[i];
        if (e.name === "mxCell") {        
          if (e.attributes) {          
            if (e.attributes.vertex === "1") { // Search vor vertex: "1"            
              var child = e.elements[0];
              for(var k = 0; k < child.elements.length; ++k) {
                var objElement = child.elements[k];
                if (objElement.name === "Object") {
                  if (objElement.attributes) {
                    if (objElement.attributes.as === "userObject") { // found "userObject"
                      // User object
                      if (objElement.elements && objElement.elements.length > 0)
                      {
                        var displayData = objElement.elements[0];
                        if (!displayData.elements) {
                          displayData.elements = [];
                        }                      
                        var vertex = {
                          id: e.attributes.id,
                          name: objElement.attributes.name || "name",
                          label: objElement.attributes.label || "label", 
                          mRID: objElement.attributes.mRID,                        
                          displayData: displayData,
                          type: objElement.attributes.type
                        };
  
                        if (e.attributes.id === this.requestCellId) {
                          this.selectedGraphItemId = this.requestCellId;
                          this.selectedGraphItem = vertex;
                        }
                        this.graphItems.push(vertex);                           
                      }                    
                    }
                  }
                }
              }            
            }
          }        
        }
      } 
      
      if (this.requestCellId) {
        
        this.selectedGraphItemId = this.requestCellId;
        this.requestCellId = null;
        this.onGraphItemChanged(this.selectedGraphItemId);
      }
    });      
  }

  onGraphItemChanged(id: string) {    
    this.currentPoints = [];
    for(var i = 0; i < this.graphItems.length; ++i) {
      if (this.graphItems[i].id === id) {
        this.selectedGraphItem = this.graphItems[i];
        this.currentPoints = this.graphItems[i].displayData.elements;
        break;
      }
    }
  }  

  removePoint(item) {
    console.log("remove data:: " + item);

    if (item) {
      for(let index = this.currentPoints.length - 1; index >=0; --index) {
        var obj = this.currentPoints[index];
        if (obj === item) {        
          this.currentPoints.splice(index, 1);
        }
      }
    }
    else {
      console.error("Unable to delete connected data point.  ID=" + id);
    }
  }

  addPoint(item: any) {
    console.log("add data:: " + item);
    this.currentPoints.push(item);
  }

  onModuleChanged(name: string) {    
    this.availablePoints = this.temp = [];
    this.filter = "";
    if (name) {
      this.profiles = getProfilesByModule(name);
    }
  }

  onProfileChanged(name: string) {
    this.filter = "";
    this.availablePoints = this.temp = [];
    if (name) {
      const currentProfile = this.profiles.find((elem) => elem.name === name);
      if (currentProfile) {
        this.availablePoints = this.temp = this.setSelectedFields(JSON.parse(JSON.stringify(currentProfile.topics)), this.availablePoints);
      }
    }
  }

  private setSelectedFields(allFields: any[], selectedFields: any[]): any[] {
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

  updateFilter(event) {
    const val = event.target.value.toLowerCase();
    
    const rows = this.temp.filter(function(d) {
      if (d.attributes) {
        if (d.attributes.label && d.attributes.label.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
        else if (d.attributes.name && d.attributes.name.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
        else if (d.attributes.path && d.attributes.path.toString().toLowerCase().indexOf(val) > -1) {
          return true;
        }
      }
    });

    this.availablePoints = rows;
  }

  updateDiagram() {
    try {
      if (this.selectedDiagramId && this.selectedGraphItemId) {      
        for(var i = 0; i < this.graphItems.length; ++i) {
          var item = this.graphItems[i];
          if (item.id === this.selectedGraphItemId) {                             
            var xml = converter.js2xml(this.graphModel);
            console.log(xml);
            this.diagram.data = xml;                        
            this.service.update(this.diagram).subscribe(
              response => {
                console.log("Updated diagram:: " + response),
                this.snack.open('Diagram is updated.', 'OK', { duration: 2000 });        
              },
              err => console.log(err)
            );            
            break;
          }
        }
      }
      else {
        console.error("Program error:: update diagram settings without selecting diagramId and vertex id.");
      }
    }
    catch (e) {
      console.error(e);
      this.snack.open('Error saving diagram.  Check logs for more information.', 'OK', { duration: 4000 });
    }
  }
}
