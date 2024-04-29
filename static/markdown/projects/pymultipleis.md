
**Pymultipleis** is a Python library for fitting a sequence of electrochemical impedance spectra. It offers a robust approach with some model function. Usually, the spectra being fitted are gradually varying or similar to each other and were obtained as a result of continuous change in the property of the electrochemical system under study. Such properties include but are not limited to temperature, potential, state of charge, and depth of discharge.
<br></br>
Implements algorithms for simultaneous and sequential fitting and leverages JAX's in-built automatic differentiation (autodiff) of Python functions. It also takes advantage of JAX's just-in-time compilation (JIT) of Python code to XLA, which runs on GPU or TPU hardware.
<br></br>
The fitting algorithm implemented in pymultieis allows the kinetic parameters of the system such as the charge transfer resistance, double layer capacitance, and Warburg coefficient to be obtained as curves which vary as a function of the dependent variable under study.
<br></br>
The 'py' in pymultipleis represents Python while the multipleis is an abbreviation for Multiple Electrochemical Impedance Spectra.
<br></br>
**pymultipleis** offers methods modules for model fitting, model validation, visualization.


