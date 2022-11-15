// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export interface Topic {
  name: string,
  mrid: string,
  value?: any,
  action?: string,
  args?: number,
  args2?: number,
}

export interface UpdateData {
  topic?: Topic
}