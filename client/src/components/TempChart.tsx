import React, { FC } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { AnimalCount } from '../App';

interface AnimalCountChartProps {
    count: AnimalCount[]
}

export const AnimalLineChart: FC<AnimalCountChartProps> = ( { count } ) => {

    return (
        <LineChart
          width={1500}
          height={800}
          data={count}
          margin={{
            top: 5,
            right: 30,
            left: 20,
            bottom: 5,
          }}
        >
          <CartesianGrid strokeDasharray="1 1" />
          <XAxis dataKey="year" />
          <YAxis />
          <Tooltip />
          <Legend />
          <Line type="monotone" dataKey="num_herbs" stroke="#8884d8" activeDot={{ r: 2 }} />
          <Line type="monotone" dataKey="num_carns" stroke="#82ca9d" />
        </LineChart>
    )
}
