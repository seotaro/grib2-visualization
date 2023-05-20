import React, { useState, forwardRef, useImperativeHandle } from 'react';

import Box from '@mui/material/Box';
import { DataGrid } from '@mui/x-data-grid';
import moment from 'moment-timezone';
import Tooltip from '@mui/material/Tooltip';
import Typography from '@mui/material/Typography';

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
    renderCell: ({ value }) => (<Tooltip title={`${moment(value).utc().format()}`} ><span>{moment(value).format('YYYY-MM-DD HH:mm')}</span></Tooltip>),
    width: 150,
    sortable: true,
  },
  {
    field: 'datetime',
    headerName: 'datetime',
    type: 'datetime',
    renderCell: ({ value }) => (<Tooltip title={`${moment(value).utc().format()}`} ><span>{moment(value).format('YYYY-MM-DD HH:mm')}</span></Tooltip>),
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
    field: 'parameter_description',
    headerName: 'parameter_description',
    type: 'string',
    width: 250,
    sortable: true,
    valueGetter: (params) => [
      params.row.parameter_description,
      params.row.discipline,
      params.row.parameter_category,
      params.row.parameter_number],
    renderCell: ({ value }) => (<Tooltip title={`${value[0]}, discipline:${value[1]}, parameter category:${value[2]}, parameter number:${value[3]}`} ><span>{value[0] ? value[0] : '(unknown)'}</span></Tooltip>),
    sortComparator: (v1, v2) => v1[0].localeCompare(v2[0]),
  },
];

export const Grib2List = forwardRef((props, ref) => {
  const { initial, onChangeSelection } = props;
  const [paginationModel, setPaginationModel] = useState({
    page: 0,
    pageSize: 25,
  });

  useImperativeHandle(ref, () => ({
    initialize: () => {
      setPaginationModel({ page: 0, pageSize: paginationModel.pageSize });
    }
  }))

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
        paginationModel={paginationModel}
        pageSizeOptions={[5, 10, 25, 50, 100]}
        onPaginationModelChange={setPaginationModel}
        headerHeight={30}
        rowHeight={30}
        hideFooterSelectedRowCount={true}
        rowSelection={true}
        rowSelectionModel={initial?.items ? [initial.selection] : []}
        onRowSelectionModelChange={(selectionModel) => onChangeSelection(selectionModel[0])}
      />
      <Box sx={{ m: 1, color: 'darkgray' }}>
        <Typography variant="body2" >
          ※ Shift + Arrow Up/Down: Select the current row and the row above or below
        </Typography>
      </Box>
    </Box>
  </>)
});
