/*******************
* Name: DessXvii
* Email: anoncomrade993@gmail.com
* GNU GENERAL PUBLIC LICENSE
* Version 3, 29 June 2007
*
* Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
* Everyone is permitted to copy and distribute verbatim copies of this license document, but changing it is not allowed.
* CREATED: 3/8/2024
*
******************/


/********
// The DCT Equation

Function DCT(image):
    N = 8  // Block size (8x8)
    
    // Initialize DCT coefficients matrix
    D = array[N][N]
    
    // Loop over each DCT coefficient (i, j)
    For i from 0 to N-1:
        For j from 0 to N-1:
            
            // Initialize the sum for this coefficient
            Sum = 0
            
            // Compute C(i) and C(j)
            Ci = (1 / sqrt(2)) if i == 0 else 1
            Cj = (1 / sqrt(2)) if j == 0 else 1
            
            // Loop over each pixel (x, y) in the block
            For x from 0 to N-1:
                For y from 0 to N-1:
                    // Compute the pixel contribution to the DCT
                    Contribution = image[x][y] * 
                                   cos(((2*x + 1) * i * PI) / (2 * N)) * 
                                   cos(((2*y + 1) * j * PI) / (2 * N))
                                   
                    // Add the contribution to the sum
                    Sum += Contribution
            
            // Compute the DCT coefficient
            D[i][j] = (1 / (2 * N)) * Ci * Cj * Sum
    
    Return D
*********/



struct Image{
  pixels :<Vec<Vec<u8>>
}

for Image{
  
  fn 
}