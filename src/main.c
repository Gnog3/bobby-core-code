#include "output.h"
#include "csr.h"

// #include "memcpy.h"

// #include "merge_sort/merge_sort.cc"



int main() {
	char* str = "Hello, world";
	printstr(0xFFFFFF, 0,  0, str);
	// printstr(0xFF0000, 0, 10, str);
	// printstr(0x00FF00, 0, 20, str);
	// printstr(0x0000FF, 0, 30, str);
	
    //char* str2 = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    //printstr(0xFF00FF, 0, 40, str2);
	
	//int arr[] = { 12, 11, 13, 5, 6, 7 };
    //int arr_size = sizeof(arr) / sizeof(arr[0]);
  

    //mergeSort(arr, 0, arr_size - 1);

	return 0;
}
