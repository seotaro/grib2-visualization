import React, { useState } from 'react';

import Box from '@mui/material/Box';
import { DataGrid } from '@mui/x-data-grid';
import moment from 'moment-timezone';
import Tooltip from '@mui/material/Tooltip';

const columns = [
  {
    field: 'id',
    headerName: 'ID',
    width: 50
  },
  {
    field: 'reference_datetime',
    headerName: 'reference_datetime',
    type: 'datetime',
    valueFormatter: ({ value }) => moment(value).format('YYYY-MM-DD HH:mm'),
    width: 150,
    sortable: true,
  },
  {
    field: 'datetime',
    headerName: 'datetime',
    type: 'datetime',
    valueFormatter: ({ value }) => moment(value).format('YYYY-MM-DD HH:mm'),
    width: 150,
    sortable: true,
  },
  {
    field: 'first_plane_name',
    headerName: 'first_plane_name',
    type: 'string',
    width: 250,
    sortable: true,
    valueGetter: (params) => [
      params.row.first_plane_name,
      params.row.first_plane_type,
      params.row.first_plane_value,
      params.row.first_plane_factor],
    renderCell: ({ value }) => (<Tooltip title={`${value[0]}, type:${value[1]}, value:${value[2]}, factor:${value[3]}`} ><span>{value[0] ? value[0] : '(unknown)'}</span></Tooltip>),
    sortComparator: (v1, v2) => v1[0].localeCompare(v2[0]),
  },
  {
    field: 'parameter_name',
    headerName: 'parameter_name',
    type: 'string',
    width: 250,
    sortable: true,
    valueGetter: (params) => [
      params.row.parameter_name,
      params.row.genre,
      params.row.parameter_category,
      params.row.parameter_number],
    renderCell: ({ value }) => (<Tooltip title={`${value[0]}, genre:${value[1]}, category:${value[2]}, number:${value[3]}`} ><span>{value[0] ? value[0] : '(unknown)'}</span></Tooltip>),
    sortComparator: (v1, v2) => v1[0].localeCompare(v2[0]),
  },
];

export const Grib2List = (props) => {
  const { initial, onChangeSelection } = props;

  return (<>
    <Box sx={{}}>
      <DataGrid
        autoHeight
        density={'compact'}
        sx={{ m: 1, }}
        rows={initial?.items ? initial.items.map((x, i) => { return { id: i, ...x } }) : []}
        columns={columns}
        initialState={{
          pagination: {
            paginationModel: { pageSize: 25, page: 0 },
          },
        }}
        pageSizeOptions={[5, 10, 25, 50, 100]}
        headerHeight={30}
        rowHeight={30}
        hideFooterSelectedRowCount={true}
        rowSelection={true}
        rowSelectionModel={initial?.items ? [initial.selection] : []}
        onRowSelectionModelChange={(selectionModel) => onChangeSelection(selectionModel[0])}
      />
      <Box sx={{ m: 1, color: 'darkgray' }}>
        ※ Shift + Arrow Up/Down: Select the current row and the row above or below
      </Box>
    </Box>
  </>)
}
