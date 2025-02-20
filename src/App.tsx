import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Calendar from 'react-calendar';
import { Table } from 'antd';
import Papa from 'papaparse';

function App() {
  const [progress, setProgress] = useState(0);
  const [dateList, setDateList] = useState<Date[]>([]);

  async function getProgress() {
    setProgress(await invoke("get_progress", {}));
  }

  async function getDateList() {
    let dateListStr: string[] = await invoke("get_synced_dates", {});
    let dateListArray = dateListStr.map((dateStr: string) => new Date(dateStr));
    setDateList(dateListArray);
  }

  useEffect(() => {
    const interval = setInterval(() => {
      getProgress();
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    getDateList();
  }, []);
  
  return (
    <div className="App" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'flex-start', height: '100vh' }}>
      <div className="status-bar" style={{ width: '80%', marginTop: '20px', textAlign: 'center', fontSize: '2em', display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
      <div
        className="progress-bar"
        style={{ width: '100%', height: '30px', margin: '0 auto', position: 'relative' }}
      >
        <div
        style={{ width: `${progress}%`, height: '100%', position: 'absolute', top: 0, left: 0 }}
        ></div>
      </div>
      <span style={{ marginTop: '10px' }}>Sync Progress: {`${Math.round(progress)}%`}</span>
      </div>
      <div style={{ margin: '20px 0' }}></div>
      <DataCalendar dateList={dateList} />
    </div>
  );
}

interface DataCalendarProps {
  dateList: Date[];
}

const DataCalendar: React.FC<DataCalendarProps> = ({ dateList }) => {
  const [abstractInfo, setAbstractInfo] = useState("");
  const [statisticsInfo, setStatisticsInfo] = useState("");

  async function handleDateChange(date: Date) {
    console.log("showing info for date", date);
    if (date.toDateString() === new Date().toDateString()) {
      const info: String = await invoke("get_today_info", {});
      console.log(info);
      setAbstractInfo("");
      setStatisticsInfo(info);
    } else {
      const year = date.getFullYear();
      const month = date.getMonth() + 1; // in js month is 0-indexed
      const day = date.getDate();
      const dateNumber = year * 10000 + month * 100 + day;
      const info: [String, String] = await invoke("get_info_by_date", { date: dateNumber });
      console.log(info[0]);
      console.log(info[1]);
      setAbstractInfo(info[0]);
      setStatisticsInfo(info[1]);
    }
  }

  return (
    <div>
      <Calendar
        onChange={handleDateChange}
        tileClassName={({ date, view }) => {
          if (view === 'month' && dateList.some(d => new Date(d).toDateString() === date.toDateString())) {
            return 'calendar-highlight';
          }
          if (view === 'month' && date.getDate() === new Date().getDate() && date.getMonth() === new Date().getMonth() && date.getFullYear() === new Date().getFullYear()) {
            return 'calendar-today';
          }
        }}
      />

      {abstractInfo && (
      <div style={{ marginTop: '20px' }}>
        <h2>Abstract Information</h2>
        <p>{abstractInfo}</p>
      </div>
      )}

      {statisticsInfo && (
      <CsvAntdTableViewer csvText={statisticsInfo} />
      )}
    </div>
  );
};

interface CsvAntdTableViewerProps {
  csvText: String;
}

const CsvAntdTableViewer: React.FC<CsvAntdTableViewerProps> = ({ csvText }) => {
  const parsedData = Papa.parse(csvText, { header: true }).data;
  const dataSource = parsedData.map((row) => ({ ...row }));
  const columns = dataSource.length > 0 ? Object.keys(dataSource[0]).map(key => ({
      title: key,
      dataIndex: key
  })) : [];

  return (
      <Table columns={columns} dataSource={dataSource} />
  );
};

export default App;
