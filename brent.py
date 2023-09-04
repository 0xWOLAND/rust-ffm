import numpy
from numpy import exp
from scipy.optimize import brentq

def brentmethod(f,xb):
    xtol=1e-15
    ytol=1e-8
    xb=numpy.array(xb,dtype="double")
    yb=numpy.array([f(xb[0]),f(xb[1])])
    # if numpy.sum(numpy.sign(yb))!=0:
    #     raise Exception("No sign change between the bounds. Brent's method requires the bounds to have different signs.")
    if numpy.abs(yb[0])<numpy.abs(yb[1]):
        xb=numpy.array([xb[1],xb[0]])
        yb=numpy.array([yb[1],yb[0]])
    c=xb[0]
    yc=yb[0]
    usedbisec=True
    while True:
        if yb[0]!=yc and yb[1]!=yc:
            s=xb[0]*yb[1]*yc/((yb[0]-yb[1])*(yb[0]-yc))+xb[1]*yb[0]*yc/((yb[1]-yb[0])*(yb[1]-yc))+c*yb[0]*yb[1]/((yc-yb[0])*(yc-yb[1]))
        else:
            s=(yb[0]*xb[1]-yb[1]*xb[0])/(yb[0]-yb[1])
        if ((s-(3*xb[0]+xb[1])/4)*(s-xb[1])>=0 or
        (usedbisec and abs(s-xb[1])>=abs(xb[1]-c)/2) or
        (not usedbisec and abs(s-xb[1])>=abs(c-d)/2) or
        (usedbisec and abs(xb[1]-c)<abs(xtol)) or
        (not usedbisec and abs(c-d)<abs(xtol))):
            s=(xb[0]+xb[1])/2
            usedbisec=True
        else:
            usedbisec=False
        ys=f(s)
        d=c
        c=xb[1]
        yc=yb[1]
        if yb[0]*ys<0:
            xb[1]=s
            yb[1]=ys
        else:
            xb[0]=s
            yb[0]=ys
        if abs(yb[0])<abs(yb[1]):
            xb=numpy.array([xb[1],xb[0]])
            yb=numpy.array([yb[1],yb[0]])
        if yb[1]==0:
            x=xb[1]
            y=yb[1]
            break
        if ys==0:
            x=s
            y=ys
            break
        if abs(xb[1]-xb[0])<xtol:
            x=s
            y=ys
            break
    if abs(y)>ytol:
        print("Warning: A pole or discontinuity may have been found.")
    return x,y


import numpy
from numpy import exp
from scipy.optimize import brentq

def disk_radial_cumulative(r):
  return (Rd**2-(Rd**2+r*Rd)*exp(-r/Rd))/Rd**2


# 'frac' is a number between 0 and 1.
def disk_radial_inverse_cumulative(frac):
  return brentq(lambda r: disk_radial_cumulative(r) - frac, 0, 1.0e10)

if __name__ == '__main__':
    Rd = 3.5
    frac = 0.5
    f=lambda r: (Rd**2-(Rd**2+r*Rd)*exp(-r/Rd))/Rd**2 
    bounds=[0,1e10]
    print(brentmethod(f,bounds))
    # print(brentq(f, 0, 1e10))
    print(disk_radial_inverse_cumulative(0.2))