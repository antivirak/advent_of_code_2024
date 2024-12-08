CREATE TABLE input_plan (
  ROW_N varchar(2),
  "0" varchar2(2),
  "1" varchar2(2),
  "2" varchar2(2),
  "3" varchar2(2),
  "4" varchar2(2),
  "5" varchar2(2),
  "6" varchar2(2),
  "7" varchar2(2),
  "8" varchar2(2),
  "9" varchar2(2),
  "10" varchar2(2),
  "11" varchar2(2),
  "12" varchar2(2),
  "13" varchar2(2),
  "14" varchar2(2),
  "15" varchar2(2),
  "16" varchar2(2),
  "17" varchar2(2),
  "18" varchar2(2),
  "19" varchar2(2),
  "20" varchar2(2),
  "21" varchar2(2),
  "22" varchar2(2),
  "23" varchar2(2),
  "24" varchar2(2),
  "25" varchar2(2),
  "26" varchar2(2),
  "27" varchar2(2),
  "28" varchar2(2),
  "29" varchar2(2),
  "30" varchar2(2),
  "31" varchar2(2),
  "32" varchar2(2),
  "33" varchar2(2),
  "34" varchar2(2),
  "35" varchar2(2),
  "36" varchar2(2),
  "37" varchar2(2),
  "38" varchar2(2),
  "39" varchar2(2),
  "40" varchar2(2),
  "41" varchar2(2),
  "42" varchar2(2),
  "43" varchar2(2),
  "44" varchar2(2),
  "45" varchar2(2),
  "46" varchar2(2),
  "47" varchar2(2),
  "48" varchar2(2),
  "49" varchar2(2)
);

CREATE SEQUENCE ROW_ID_SEQ MINVALUE 0 MAXVALUE 99999999 INCREMENT BY 1 START WITH 0 CACHE 20 NOORDER NOCYCLE;

create or replace TRIGGER row_id_trig
  BEFORE INSERT ON input_plan
  FOR EACH ROW
  WHEN (new.ROW_n IS NULL)
  BEGIN
    SELECT ROW_ID_SEQ.NEXTVAL
    INTO   :new.row_n
    FROM   dual;
  END;
--/


-- test input:
insert ALL
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','0','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','0','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','0','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','0','.','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','A','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','A','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','A','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','.','.','.')
INTO input_plan("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11") VALUES ('.','.','.','.','.','.','.','.','.','.','.','.')
SELECT 1 FROM DUAL;

CREATE TABLE antennas (antenna varchar(2), x number(8), y number(8));

declare
    l_sql varchar(200);
    col_n varchar(2);
BEGIN
    SELECT max(CAST(row_n AS NUMBER(8))) INTO col_n from input_plan;
    for i in 0..col_n LOOP
        for j in 0..col_n LOOP
            l_sql := 'INSERT INTO antennas(antenna, x, y) VALUES ((select "' || j || '" from input_plan where row_n = ' || i || '),''' || i || ''', ''' || j || ''')';
            EXECUTE IMMEDIATE l_sql;
        end loop;
    end loop;
end;
--/

delete FROM antennas WHERE antenna = '.';

--part 1
SELECT count(*) FROM (
  SELECT pair1_x - (pair0_x - pair1_x) xnew, pair1_y - (pair0_y - pair1_y) ynew from (
    SELECT A1.antenna, A1.x AS pair0_x, A1.y pair0_y, A2.x pair1_x, A2.y pair1_y
    FROM (SELECT DISTINCT antenna, x, y FROM antennas) A1
    inner JOIN (SELECT DISTINCT antenna, x, y FROM antennas) A2 ON A1.antenna = A2.antenna
    WHERE A1.x != A2.x AND A1.y != A2.y
  ) WHERE pair1_x - (pair0_x - pair1_x) >= 0 AND pair1_y - (pair0_y - pair1_y) >= 0
  AND pair1_x - (pair0_x - pair1_x) <= (SELECT max(CAST(row_n as NUMBER(8))) FROM input_plan)
  AND pair1_y - (pair0_y - pair1_y) <= (SELECT max(CAST(row_n as NUMBER(8))) FROM input_plan)
UNION
  SELECT pair0_x + (pair0_x - pair1_x) xnew, pair0_y + (pair0_y - pair1_y) ynew from (
    SELECT A1.antenna, A1.x AS pair0_x, A1.y pair0_y, A2.x pair1_x, A2.y pair1_y
    FROM (SELECT DISTINCT antenna, x, y FROM antennas) A1
    inner JOIN (SELECT DISTINCT antenna, x, y FROM antennas) A2 ON A1.antenna = A2.antenna
    WHERE A1.x != A2.x AND A1.y != A2.y
  ) WHERE pair0_x + (pair0_x - pair1_x) >= 0 AND pair0_y + (pair0_y - pair1_y) >= 0
  AND pair0_x + (pair0_x - pair1_x) <= (SELECT max(CAST(row_n as NUMBER(8))) FROM input_plan)
  AND pair0_y + (pair0_y - pair1_y) <= (SELECT max(CAST(row_n as NUMBER(8))) FROM input_plan)
);
 
-- part 2
DECLARE
    res number(8);
    l_sql varchar(32767);
BEGIN
    l_sql := 'with m as (SELECT max(CAST(row_n as NUMBER(8))) mx_len FROM input_plan),
a as (SELECT A1.antenna,A1.x pair0_x,A1.y pair0_y,A2.x pair1_x,A2.y pair1_y
FROM (SELECT antenna,x,y FROM antennas) A1
inner JOIN (SELECT antenna,x,y FROM antennas) A2 ON A1.antenna=A2.antenna
WHERE A1.x!=A2.x AND A1.y!=A2.y)
select count(*) from (SELECT pair1_x,pair1_y from (SELECT A1.antenna,A1.x pair0_x,A1.y pair0_y,A2.x pair1_x,A2.y pair1_y
FROM (SELECT DISTINCT antenna,x,y FROM antennas) A1
inner JOIN (SELECT DISTINCT antenna,x,y FROM antennas) A2 ON A1.antenna=A2.antenna
WHERE A1.x != A2.x AND A1.y != A2.y)';
    FOR i IN 1..43 LOOP
        l_sql := l_sql || ' union SELECT pair1_x-(pair0_x-pair1_x)*'||i||',pair1_y-(pair0_y-pair1_y)*'||i||' from m,a
WHERE pair1_x-(pair0_x-pair1_x)*'||i||'>=0 AND pair1_y-(pair0_y-pair1_y)*'||i||'>=0
AND pair1_x-(pair0_x-pair1_x)*'||i||'<=m.mx_len AND pair1_y-(pair0_y-pair1_y)*'||i||'<=m.mx_len
UNION
SELECT pair0_x+(pair0_x-pair1_x)*'||i||',pair0_y+(pair0_y-pair1_y)*'||i||' from m,a
WHERE pair0_x+(pair0_x-pair1_x)*'||i||'>=0 AND pair0_y+(pair0_y-pair1_y)*'||i||'>=0
AND pair0_x+(pair0_x-pair1_x)*'||i||'<=m.mx_len AND pair0_y+(pair0_y-pair1_y)*'||i||'<=m.mx_len';
    END LOOP;
    EXECUTE immediate l_sql || ')' INTO res;
    dbms_output.put_line('part2 solution: ' || res);
END;
--/


DROP TABLE antennas;

DROP TABLE input_plan;

DROP sequence ROW_ID_SEQ;
