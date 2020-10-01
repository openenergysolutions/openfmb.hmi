import { Action, createReducer, on } from '@ngrx/store';
import { DesignerConstant } from '../../core/constants/designer-constant';
import * as designerActions from '../actions/designer.actions';

export const designerFeatureKey = 'designer';

export interface State {
  mode: number;
  connectColor: string;
}

export const initialState: State = {
  mode: DesignerConstant.SELECT_MODE,
  connectColor: DesignerConstant.CONNECT_COLORS[0]
};

const designerReducer = createReducer(
  initialState,
  on(
    designerActions.selectMode,
    (state, { mode }) => ({
      ...state,
      mode
    })
  ),
  on(
    designerActions.selectColor,
    (state, { connectColor }) => ({
      ...state,
      connectColor
    })
  )
);

export function reducer(state: State | undefined, action: Action) {
  return designerReducer(state, action);
}
