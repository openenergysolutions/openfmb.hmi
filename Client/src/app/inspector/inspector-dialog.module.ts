import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatSortModule } from '@angular/material/sort';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { SharedMaterialModule } from '../shared/shared-material.module';
import { InspectorDialogRoutingModule } from './inspector-dialog-routing.module';
import { InspectorDialogComponent } from './inspector-dialog.component';


@NgModule({
  declarations: [InspectorDialogComponent],
  imports: [
    CommonModule,
    MatInputModule,
    MatPaginatorModule,
    MatSortModule,
    MatTableModule,
    SharedMaterialModule,
    InspectorDialogRoutingModule
  ]
})
export class InspectorDialogModule { }
