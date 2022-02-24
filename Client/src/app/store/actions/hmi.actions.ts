// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { createAction, props } from '@ngrx/store';

export const commStatus = createAction(
    '[HMI] Comm Status',
    props<{ status: number }>()
);