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
            time_stream : out std_logic_vector(63 downto 0);
			a_stream : out std_logic_vector(31 downto 0);
			b_stream : out std_logic_vector(31 downto 0);
			c_stream : out std_logic_vector(31 downto 0);
			d_stream : out std_logic_vector(31 downto 0);
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
	signal b_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal c_data_out : std_logic_vector(31 downto 0) := (others => '0');
	signal d_data_out : std_logic_vector(31 downto 0) := (others => '0');
    signal new_input_in : std_logic := '0';
    signal lost_data : std_logic := '0';

    -- constants
    signal const1 : std_logic := '1';

    -- config signals
    constant new_input_clock_cycle : natural := 10;
    constant number_iterations : natural := 2*30;
    constant number_inputs : natural := 8;

    -- set inputs
    -- current time unit: nanoseconds -> time given in milliseconds
    constant time_unit_multiplication : unsigned(31 downto 0) := to_unsigned(1000000, 32);
	constant time_test_data : unsigned32_array(0 to 20) := (
		to_unsigned(0000,32),
		to_unsigned(10, 32),
		to_unsigned(110, 32),
		to_unsigned(210, 32),
		to_unsigned(310, 32),
		to_unsigned(410, 32),
		to_unsigned(510, 32),
		to_unsigned(610, 32),
		to_unsigned(710, 32),
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
	constant new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0');
	constant a_test_data : signed32_array(0 to 20) := (
		to_signed(1, a_data'length),
		to_signed(2, a_data'length),
		to_signed(1, a_data'length),
		to_signed(2, a_data'length),
		to_signed(1, a_data'length),
		to_signed(2, a_data'length),
		to_signed(1, a_data'length),
		to_signed(2, a_data'length),
		to_signed(1, a_data'length),
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
	constant a_new_input_test_data : bit_array(0 to 20) := ('1','1','1','1','1','1','1','1','1','0','0','0','0','0','0','0','0','0','0','0','0');
	constant exp_b_data : signed32_array(0 to 20) := (
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(2, b_data_out'length),
		to_signed(1, b_data_out'length),
		to_signed(2, b_data_out'length),
		to_signed(1, b_data_out'length),
		to_signed(2, b_data_out'length),
		to_signed(1, b_data_out'length),
		to_signed(2, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length),
		to_signed(0, b_data_out'length)
	);
	constant exp_c_data : signed32_array(0 to 20) := (
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(1, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length),
		to_signed(0, c_data_out'length)
	);
	constant exp_d_data : signed32_array(0 to 20) := (
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(2, d_data_out'length),
		to_signed(2, d_data_out'length),
		to_signed(2, d_data_out'length),
		to_signed(2, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length),
		to_signed(0, d_data_out'length)
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
            time_stream => time_stream,
					a_stream => a_data_out,
				b_stream => b_data_out,
				c_stream => c_data_out,
				d_stream => d_data_out,
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
            end if;
            for I_inner in 1 to number_iterations loop
                clk <= '1';
                wait for 1 ps;
                clk <= '0';
                wait for 1 ps;
            end loop;
            new_input_in <= '0';
			a_data_new_input <= '0';
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
				assert b_data_out = std_logic_vector(exp_b_data(I))
					report "b_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert c_data_out = std_logic_vector(exp_c_data(I))
					report "c_data differ at iteration I = " & integer'image(I)
					severity Error;
				assert d_data_out = std_logic_vector(exp_d_data(I))
					report "d_data differ at iteration I = " & integer'image(I)
					severity Error;
            end if;
        end loop;
        wait;
    end process;

end behaviour;
