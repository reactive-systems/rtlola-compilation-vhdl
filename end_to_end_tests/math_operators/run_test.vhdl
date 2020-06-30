library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use work.array_type_pkg.all;

entity run_test is
end entity run_test;

architecture behaviour of run_test is

    component implementation is
        port(
            clk, rst : in std_logic;
            offline : in std_logic;
            input_time : in std_logic_vector(63 downto 0);
            new_input: in std_logic;
			a_data_in : in std_logic_vector(31 downto 0);
			a_data_in_new_input : in std_logic;
			b_data_in : in std_logic_vector(31 downto 0);
			b_data_in_new_input : in std_logic;
            time_stream : out std_logic_vector(63 downto 0);
			a_stream : out std_logic_vector(31 downto 0);
			b_stream : out std_logic_vector(31 downto 0);
			plus_op_stream : out std_logic_vector(31 downto 0);
			minus_op_stream : out std_logic_vector(31 downto 0);
			mult_op_stream : out std_logic_vector(31 downto 0);
			div_op_stream: out std_logic_vector(31 downto 0);
			mod_op_stream : out std_logic_vector(31 downto 0);
			-- pow_op_stream : out std_logic_vector(31 downto 0);
			func_abs_stream : out std_logic_vector(31 downto 0);
			func_sqrt_stream : out std_logic_vector(31 downto 0);
			counter_stream : out std_logic_vector(31 downto 0);
            lost_data : out std_logic
        );
    end component;

    signal clk : std_logic := '0';
    signal rst : std_logic := '1';
    signal time_data : std_logic_vector(63 downto 0) := (others => '0');
    signal time_stream : std_logic_vector(63 downto 0);
	signal a_data : std_logic_vector(31 downto 0) := (others => '0');
	signal a_data_new_input : std_logic := '0';
	signal a_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal b_data : std_logic_vector(31 downto 0) := (others => '0');
	signal b_data_new_input : std_logic := '0';
	signal b_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal plus_op_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal minus_op_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal mult_op_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal div_op_data_out: std_logic_vector(31 downto 0) := (others => '0');
	signal mod_op_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal pow_op_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal func_abs_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal func_sqrt_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal counter_data_out : std_logic_vector(31 downto 0) := (others => '0');
    signal new_input_in : std_logic := '0';
    signal lost_data : std_logic := '0';

    -- constants
    signal const1 : std_logic := '1';

    -- config signals
    constant new_input_clock_cycle : natural := 10;
    constant number_iterations : natural := 2*20;
    constant number_inputs : natural := 5;

    -- set inputs
    -- current time unit: nanoseconds -> time given in milliseconds
    constant time_unit_multiplication : unsigned(31 downto 0) := to_unsigned(1000000, 32);
	constant time_test_data : unsigned32_array(0 to 20) := (
		to_unsigned(0000,32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32),
		to_unsigned(0000, 32)
	);
	constant new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant a_test_data : signed32_array(0 to 20) := (
		to_signed(1, a_data'length),
		to_signed(6, a_data'length),
		to_signed(9, a_data'length),
		to_signed(10, a_data'length),
		to_signed(5, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length),
		to_signed(0, a_data'length)
	);
	constant a_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant b_test_data : signed32_array(0 to 20) := (
		to_signed(5, b_data'length),
		to_signed(3, b_data'length),
		to_signed(3, b_data'length),
		to_signed(-2, b_data'length),
		to_signed(1, b_data'length),
		to_signed(8, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length),
		to_signed(0, b_data'length)
	);
	constant b_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0','0','0','0');
	constant exp_plus_op_data : signed32_array(0 to 20) := (
		to_signed(6, plus_op_data_out'length),
		to_signed(9, plus_op_data_out'length),
		to_signed(12, plus_op_data_out'length),
		to_signed(8, plus_op_data_out'length),
		to_signed(6, plus_op_data_out'length),
		to_signed(8, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length),
		to_signed(0, plus_op_data_out'length)
	);
	constant exp_minus_op_data : signed32_array(0 to 20) := (
		to_signed(-4, minus_op_data_out'length),
		to_signed(3, minus_op_data_out'length),
		to_signed(6, minus_op_data_out'length),
		to_signed(12, minus_op_data_out'length),
		to_signed(4, minus_op_data_out'length),
		to_signed(-8, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length),
		to_signed(0, minus_op_data_out'length)
	);
	constant exp_mult_op_data : signed32_array(0 to 20) := (
		to_signed(5, mult_op_data_out'length),
		to_signed(18, mult_op_data_out'length),
		to_signed(27, mult_op_data_out'length),
		to_signed(-20, mult_op_data_out'length),
		to_signed(5, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length),
		to_signed(0, mult_op_data_out'length)
	);

	constant exp_div_op_data : signed32_array(0 to 20) := (
		to_signed(0, div_op_data_out'length),
		to_signed(2, div_op_data_out'length),
		to_signed(3, div_op_data_out'length),
		to_signed(-5, div_op_data_out'length),
		to_signed(5, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length),
		to_signed(0, div_op_data_out'length)
	);
	
	constant exp_mod_op_data : signed32_array(0 to 20) := (
		to_signed(1, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length),
		to_signed(0, mod_op_data_out'length)
	);
	constant exp_pow_op_data : signed32_array(0 to 20) := (
		to_signed(1, pow_op_data_out'length),
		to_signed(-216, pow_op_data_out'length),
		to_signed(729, pow_op_data_out'length),
		to_signed(64, pow_op_data_out'length),
		to_signed(5, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length),
		to_signed(0, pow_op_data_out'length)
	);
	constant exp_func_abs_data : signed32_array(0 to 20) := (
		to_signed(5, func_abs_data_out'length),
		to_signed(3, func_abs_data_out'length),
		to_signed(3, func_abs_data_out'length),
		to_signed(2, func_abs_data_out'length),
		to_signed(1, func_abs_data_out'length),
		to_signed(8, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length),
		to_signed(0, func_abs_data_out'length)
	);
	constant exp_func_sqrt_data : signed32_array(0 to 20) := (
		to_signed(1, func_sqrt_data_out'length),
		to_signed(2, func_sqrt_data_out'length),
		to_signed(3, func_sqrt_data_out'length),
		to_signed(3, func_sqrt_data_out'length),
		to_signed(2, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length),
		to_signed(0, func_sqrt_data_out'length)
	);
	constant exp_counter_data : signed32_array(0 to 20) := (
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length),
		to_signed(0, counter_data_out'length)
	);

begin

    implementation_instance: implementation
        port map (
            clk => clk,
            rst => rst,
            offline => const1,
            input_time => time_data,
            new_input => new_input_in,
					a_data_in => a_data,
					a_data_in_new_input => a_data_new_input,
					b_data_in => b_data,
					b_data_in_new_input => b_data_new_input,
            time_stream => time_stream,
					a_stream => a_data_out,
					b_stream => b_data_out,
				plus_op_stream => plus_op_data_out,
				minus_op_stream => minus_op_data_out,
				mult_op_stream => mult_op_data_out,
				div_op_stream => div_op_data_out,
				mod_op_stream => mod_op_data_out,
				-- pow_op_stream => pow_op_data_out,
				func_abs_stream => func_abs_data_out,
				func_sqrt_stream => func_sqrt_data_out,
				counter_stream => counter_data_out,
            lost_data => lost_data
        );

    process

    begin
        -- reset to get initial values
        rst <= '0';
        wait for 1 ps;
        -- set number of repetitions
        for I in 1 to new_input_clock_cycle loop
            clk <= '1';
            wait for 1 ps;
            clk <= '0';
            wait for 1 ps;
        end loop;
        -- set number of repetitions
        for I in 0 to 2*number_inputs loop
            --set inputs for each iteration
            if (I <= number_inputs) then
				time_data <= std_logic_vector(time_test_data(I) * time_unit_multiplication);
				new_input_in <= new_input_test_data(I);
				a_data <= std_logic_vector(a_test_data(I));
				a_data_new_input <= a_new_input_test_data(I);
				b_data <= std_logic_vector(b_test_data(I));
				b_data_new_input <= b_new_input_test_data(I);
            end if;
            for I_inner in 1 to number_iterations loop
                clk <= '1';
                wait for 1 ps;
                clk <= '0';
                wait for 1 ps;
            end loop;
            new_input_in <= '0';
			a_data_new_input <= '0';
			b_data_new_input <= '0';
            for I_inner in 1 to 20*number_iterations loop
                clk <= '1';
                wait for 1 ps;
                clk <= '0';
                wait for 1 ps;
            end loop;
            if (I <= number_inputs) then
				assert a_data_out = a_data
					report "a_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert b_data_out = b_data
					report "b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert plus_op_data_out = std_logic_vector(exp_plus_op_data(I))
					report "plus_op_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert minus_op_data_out = std_logic_vector(exp_minus_op_data(I))
					report "minus_op_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert mult_op_data_out = std_logic_vector(exp_mult_op_data(I))
					report "mul_op_data differ at iteration I = " & integer'image(I)
					severity Error;
					assert div_op_data_out = std_logic_vector(exp_div_op_data(I))
					report "div_op_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert mod_op_data_out = std_logic_vector(exp_mod_op_data(I))
					report "mod_op_data differ at iteration I = " & integer'image(I)
					severity Error;
				-- assert pow_op_data_out = std_logic_vector(exp_pow_op_data(I))
				-- 	report "pow_op_data differ at iteration I = " & integer'image(I)
				-- 	severity Error;
				assert func_abs_data_out = std_logic_vector(exp_func_abs_data(I))
					report "func_abs_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert func_sqrt_data_out = std_logic_vector(exp_func_sqrt_data(I))
					report "func_sqrt_data differ at iteration I = " & integer'image(I)
					severity Error;
				-- assert counter_data_out = std_logic_vector(exp_counter_data(I))
				-- 	report "counter_data differ at iteration I = " & integer'image(I)
				-- 	severity Error;
            end if;
        end loop;
        wait;
    end process;

end behaviour;
