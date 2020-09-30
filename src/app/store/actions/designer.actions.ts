import { createAction, props } from '@ngrx/store';

export const selectMode = createAction(
  '[Designer] Select Mode',
  props<{ mode: number }>()
);

export const selectColor = createAction(
  '[Designer] Select Color',
  props<{ connectColor: string }>()
);

export const saveGraph = createAction(
  '[Designer] Save Graph',
  props<{ connectColor: string }>()
);
