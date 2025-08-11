using System;
using System.Runtime.InteropServices;

namespace dotnet_project {
    public static class Program {
        public delegate void HelloWorld1Delegate();
        public static void HelloWorld1() {
            Console.WriteLine("Hello 1 from C#!");
        }

        [UnmanagedCallersOnly]
        public static void HelloWorld2() {
            Console.WriteLine("Hello 2 from C#!");
        }
        
        public static int HelloWorld3(IntPtr arg, int argLength) {
            Console.WriteLine("Hello 3 from C#!");
            return 0;
        }
    }
}