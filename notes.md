[http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf#%5B%7B%22num%22%3A299%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C67%2C338%2Cnull%5D](manual link)

#Enabeling GPIO pins:
GPIO pins must be enabeled in 2 steps. The first is to change the correct peripheral 
enable register to 1. 
##Enabeling the GPIO peripheral
The AHB register is part of the Rcc block.
See section 9.4.6 in the datasheet for details.

##Setting the pin as an output
You also need to configure the direction of the specific pin you want to use. This is
done by writing to the MODER register for that GPIO pin. See section 11.4.1 in the 
manual

