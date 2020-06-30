library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.fixed_pkg.all;

entity implementation is
    port (
        clk: in std_logic;
        rst: in std_logic;
        offline : in std_logic;
        input_time: in std_logic_vector(63 downto 0);
        new_input : in std_logic;
		a_data_in : in std_logic_vector(31 downto 0);
		a_data_in_new_input : in std_logic;
        time_stream : out std_logic_vector(63 downto 0);
		a_stream: out std_logic_vector(31 downto 0);
		s_s_stream: out std_logic_vector(31 downto 0);
		c_s_stream: out std_logic_vector(63 downto 0);
		av_s_stream: out std_logic_vector(31 downto 0);
		a_u_stream: out std_logic_vector(31 downto 0);
		s_u_stream: out std_logic_vector(31 downto 0);
		c_u_stream: out std_logic_vector(63 downto 0);
		av_u_stream: out std_logic_vector(31 downto 0);
        lost_data : out std_logic
    );
end entity;

architecture structural of implementation is

    component clock_pre_processing is
        port (
            clk : in std_logic;
            rst : in std_logic;
            sys_clk : out std_logic;
            tclk : out std_logic;
            eclk : out std_logic
        );
    end component;

    component input_pre_processing is
        port (
            clk : in std_logic;
            rst : in std_logic;
            time_in : in std_logic_vector(63 downto 0);
            new_input_in : in std_logic;
			a_in : in std_logic_vector(31 downto 0);
			a_in_new_input : in std_logic;
            time_out : out std_logic_vector(63 downto 0);
			a_out : out std_logic_vector(31 downto 0);
			a_out_new_input : out std_logic;
            new_input_out : out std_logic
        );
    end component;

    component monitor is
        port (
            clk, tclk, qclk, eclk, rst : in std_logic;
            input_time : in std_logic_vector(63 downto 0);
            offline : in std_logic;
            new_input : in std_logic;
			a_data_in : in std_logic_vector(31 downto 0);
			a_data_in_new_input : in std_logic;
            time_stream : out std_logic_vector(63 downto 0);
			a_stream: out std_logic_vector(31 downto 0);
			s_s_stream: out std_logic_vector(31 downto 0);
			c_s_stream: out std_logic_vector(63 downto 0);
			av_s_stream: out std_logic_vector(31 downto 0);
			a_u_stream: out std_logic_vector(31 downto 0);
			s_u_stream: out std_logic_vector(31 downto 0);
			c_u_stream: out std_logic_vector(63 downto 0);
			av_u_stream: out std_logic_vector(31 downto 0);
            lost_data: out std_logic
        );
    end component;

    signal sys_clk : std_logic;
    signal tclk : std_logic;
    signal eclk : std_logic;
    signal time_reg : std_logic_vector(63 downto 0);
    signal new_input_reg : std_logic;
	signal a_data_reg : std_logic_vector(31 downto 0);
	signal a_data_reg_new_input : std_logic;

begin

    clock_pre_processing_instance: clock_pre_processing
        port map (
            clk => clk,
            rst => rst,
            sys_clk => sys_clk,
            tclk => tclk,
            eclk => eclk
        );

    input_pre_processing_instance: input_pre_processing
        port map (
            clk => clk,
            rst => rst,
            time_in => input_time,
            new_input_in => new_input,
			a_in => a_data_in,
			a_in_new_input => a_data_in_new_input,
            time_out => time_reg,
			a_out => a_data_reg,
			a_out_new_input => a_data_reg_new_input,
            new_input_out => new_input_reg
        );

    monitor_instance: monitor
        port map (
            clk => sys_clk,
            tclk => tclk,
            qclk => sys_clk,
            eclk => eclk,
            rst => rst,
            input_time => time_reg,
            offline => offline,
            new_input => new_input_reg,
			a_data_in => a_data_reg,
			a_data_in_new_input => a_data_reg_new_input,
            time_stream => time_stream,
			a_stream => a_stream,
			s_s_stream => s_s_stream,
			c_s_stream => c_s_stream,
			av_s_stream => av_s_stream,
			a_u_stream => a_u_stream,
			s_u_stream => s_u_stream,
			c_u_stream => c_u_stream,
			av_u_stream => av_u_stream,
            lost_data => lost_data
        );

end structural;