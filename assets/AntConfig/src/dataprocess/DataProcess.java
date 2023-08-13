package dataprocess;

/**
 *
 * @author esteb
 */

import java.util.Arrays;
import javax.swing.JOptionPane;

public class DataProcess {

    public static void main(String[] args) throws Exception {
        new DataProcess().operator();
    }

    public void operator() {
        int arraySize;
        while (true) {
            try {
                arraySize = Integer.parseInt(JOptionPane.showInputDialog("Introduce the length of the array"));
                if (arraySize <= 0) throw new NumberFormatException();
                break;
            } catch (NumberFormatException e) {
                JOptionPane.showMessageDialog(null, "Invalid input. Please enter a positive integer.");
            }
        }

        float[] array = new float[arraySize];

        for (int i = 0; i < arraySize; i++) {
            while (true) {
                try {
                    array[i] = Float.parseFloat(JOptionPane.showInputDialog("Introduce the value of the element " + i));
                    break;
                } catch (NumberFormatException e) {
                    JOptionPane.showMessageDialog(null, "Invalid input. Please enter a float.");
                }
            }
        }

        Arrays.sort(array);

        float sum = 0;
        for (float value : array) {
            sum += value;
        }
        float average = sum / array.length;

        sum = 0;
        for (float value : array) {
            sum += Math.pow(value - average, 2);
        }
        float standardDeviation = (float) Math.sqrt(sum / array.length);

        JOptionPane.showMessageDialog(null, "The array is " + Arrays.toString(array));
        JOptionPane.showMessageDialog(null, "The maximum value is " + array[array.length - 1]);
        JOptionPane.showMessageDialog(null, "The minimum value is " + array[0]);
        JOptionPane.showMessageDialog(null, "The average is " + average);
        JOptionPane.showMessageDialog(null, "The standard deviation is " + standardDeviation);

        // Display n greatest elements
        int n;
        while (true) {
            try {
                n = Integer.parseInt(JOptionPane.showInputDialog("Input the n greatest elements you want to see"));
                if (n <= 0 || n > array.length) throw new NumberFormatException();
                break;
            } catch (NumberFormatException e) {
                JOptionPane.showMessageDialog(null, "Invalid input. Please enter a positive integer less than or equal to the array length.");
            }
        }
        float[] nMaxElements = new float[n];
        for (int i = 0; i < n; i++) {
            nMaxElements[i] = array[array.length - 1 - i];
        }
        JOptionPane.showMessageDialog(null, "The " + n + " greatest elements are " + Arrays.toString(nMaxElements));

        // Display n smallest elements
        while (true) {
            try {
                n = Integer.parseInt(JOptionPane.showInputDialog("Input the n smallest elements you want to see"));
                if (n <= 0 || n > array.length) throw new NumberFormatException();
                break;
            } catch (NumberFormatException e) {
                JOptionPane.showMessageDialog(null, "Invalid input. Please enter a positive integer less than or equal to the array length.");
            }
        }
        float[] nMinElements = new float[n];
        for (int i = 0; i < n; i++) {
            nMinElements[i] = array[i];
        }
        JOptionPane.showMessageDialog(null, "The " + n + " smallest elements are " + Arrays.toString(nMinElements));
    }
}
