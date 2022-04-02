f(x) = a*x+b
fit f(x) "data.sim" u 2:3 via a,b
 plot "data.sim" using 2:3 with points, f(x) with lines;
pause -1;
